use shared_data::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS, encode_v1};
use std::{collections::VecDeque, io::Write, sync::mpsc::Sender, time::Instant};
use thiserror::Error;

fn get_uuid() -> u128 {
    let path = std::path::Path::new("uuid");
    if path.exists() {
        let contents = std::fs::read_to_string(path).unwrap();
        contents.parse::<u128>().unwrap()
    } else {
        let uuid = uuid::Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string()).unwrap();
        uuid
    }
}

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect to the server")]
    UnableToConnect,

    #[error("Sending the data failed")]
    UnableToSend,
}

pub fn collect_data(tx: Sender<CollectorCommandV1>, collector_id: u128) {
    // Initialize the sysinfo system
    let mut sys = sysinfo::System::new_all();

    // Perform a single refresh and pause. `sysinfo` gathers data via deltas,
    // and the first reading is usually useless.
    sys.refresh_memory();
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));

    // Run forever
    loop {
        // Note when we're starting
        let now = Instant::now();

        // Refresh the stored data
        sys.refresh_memory();
        sys.refresh_cpu_all();

        // Get new values
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let num_cpus = sys.cpus().len();
        let total_cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();
        let average_cpu_usage = total_cpu_usage / num_cpus as f32;

        // Submit
        let send_result = tx.send(CollectorCommandV1::SubmitData {
            collector_id,
            total_memory,
            used_memory,
            average_cpu_usage,
        });
        if let Err(e) = send_result {
            println!("Error sending data: {e:?}");
        }

        // Wait for the next cycle
        let elapsed_seconds = now.elapsed().as_secs_f32();
        if elapsed_seconds < 1.0 {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 - elapsed_seconds));
        } else {
            // Warning: we're running behind!
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
        }
    }
}

pub fn send_command(bytes: &[u8]) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(bytes)
        .map_err(|_| CollectorError::UnableToSend)?;
    Ok(())
}

pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    // Connect
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;

    // Send every queue item
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSend);
        }
    }

    Ok(())
}

fn main() {
    let uuid = get_uuid();
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        collect_data(tx, uuid);
    });

    // Listen for commands to send
    let mut data_queue = VecDeque::with_capacity(120);
    while let Ok(command) = rx.recv() {
        let encoded = encode_v1(&command);
        data_queue.push_back(encoded);
        let _ = send_queue(&mut data_queue);

        // // Send all the queued commands
        // while let Some(command) = send_queue.pop_front() {
        //     if send_command(&command).is_err() {
        //         println!("Error sending command");
        //         send_queue.push_front(command);
        //         break;
        //     }
        // }
        // let _ = send_command(command)
    }
}

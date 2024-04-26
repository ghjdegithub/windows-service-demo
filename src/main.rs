#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    use std::env;
    use std::ffi::OsStr;
    use std::io;
    use std::io::Read;
    use windows_service::{
        service::ServiceAccess,
        service::ServiceState,
        service_manager::{ServiceManager, ServiceManagerAccess},
    };

    let service_name = env::args().nth(1).expect("no arg service");
    let service_name = service_name.as_str();

    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service = service_manager.open_service(service_name, ServiceAccess::QUERY_CONFIG)?;
    let config = service.query_config()?;
    println!("{:#?}", config);

    let service_query = service_manager.open_service(service_name, ServiceAccess::QUERY_STATUS)?;

    let status = service_query.query_status()?;
    println!("{:#?}", status);

    if let ServiceState::Running = status.current_state {
        let service = service_manager.open_service(service_name, ServiceAccess::STOP)?;
        service.stop()?;
        loop {
            let status = service_query.query_status()?;
            println!("{:#?}", status);
            if let ServiceState::Stopped = status.current_state {
                println!("stop success!");
                break;
            }
        }
    }

    let status = service_query.query_status()?;
    if let ServiceState::Stopped = status.current_state {
        let service = service_manager.open_service(service_name, ServiceAccess::START)?;
        service.start(&[OsStr::new("Started from Rust!")])?;
        loop {
            let status = service_query.query_status()?;
            println!("{:#?}", status);
            if let ServiceState::Running = status.current_state {
                println!("start success!");
                break;
            }
        }
    }

    println!("按回车键退出...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}

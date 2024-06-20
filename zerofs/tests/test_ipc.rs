#![cfg(unix)]

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[test]
fn test_ipc_open_at() -> anyhow::Result<()> {
    procspawn::init();
    let process_1 = procspawn::spawn!(
        () || {
            println!("Hello from a child process 1!");
        }
    );

    let process_2 = procspawn::spawn!(
        () || {
            println!("Hello from a child process 2!");
        }
    );

    process_1.join()?;
    process_2.join()?;

    Ok(())
}

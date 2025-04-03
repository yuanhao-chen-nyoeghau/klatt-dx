fn main() -> std::io::Result<()> {
    std::process::Command::new("npx")
        .arg("--yes")
        .arg("@tailwindcss/cli")
        .arg("-i")
        .arg("./input.css")
        .arg("-o")
        .arg("./tailwind.css")
        .status()?;

    Ok(())
}

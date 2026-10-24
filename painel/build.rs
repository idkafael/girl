fn main() {
    // Coloca o ícone da Girl no .exe (só no Windows).
    #[cfg(windows)]
    embed_resource::compile("app.rc", embed_resource::NONE);
}

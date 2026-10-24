// Painel da Girl Agência
// Um lançador: abre um navegador isolado (perfil próprio) por criadora.
// Sem proxy, sem fingerprint — só perfis separados do Chrome/Edge.

#![windows_subsystem = "windows"]

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf, process::Command, sync::Arc};

// ---------- cores da marca ----------
const PRETO: egui::Color32 = egui::Color32::from_rgb(14, 14, 14);
const CARD: egui::Color32 = egui::Color32::from_rgb(23, 23, 23);
const ROSA: egui::Color32 = egui::Color32::from_rgb(230, 0, 126);
const CREME: egui::Color32 = egui::Color32::from_rgb(243, 233, 210);

// ---------- dados ----------
#[derive(Serialize, Deserialize, Clone)]
struct Criadora {
    nome: String,
    #[serde(default = "abas_padrao")]
    abas: Vec<String>,
}

fn abas_padrao() -> Vec<String> {
    vec![
        "https://www.instagram.com/".to_string(),
        "https://privacy.com.br/".to_string(),
        "https://web.telegram.org/".to_string(),
    ]
}

fn base_dir() -> PathBuf {
    env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn json_path() -> PathBuf {
    base_dir().join("criadoras.json")
}

fn carregar() -> Vec<Criadora> {
    match fs::read_to_string(json_path()) {
        Ok(txt) => serde_json::from_str(&txt).unwrap_or_default(),
        Err(_) => {
            let inicial = vec![Criadora {
                nome: "Lua".to_string(),
                abas: abas_padrao(),
            }];
            salvar(&inicial);
            inicial
        }
    }
}

fn salvar(lista: &[Criadora]) {
    if let Ok(txt) = serde_json::to_string_pretty(lista) {
        let _ = fs::write(json_path(), txt);
    }
}

fn slug(nome: &str) -> String {
    let s: String = nome
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let s = s.trim_matches('-').to_string();
    if s.is_empty() {
        "perfil".to_string()
    } else {
        s
    }
}

fn data_dir() -> PathBuf {
    let lad = env::var("LOCALAPPDATA").unwrap_or_default();
    PathBuf::from(format!("{lad}\\GirlPainel\\Chrome"))
}

fn achar_navegador() -> Option<PathBuf> {
    let pf = env::var("ProgramFiles").unwrap_or_default();
    let pfx = env::var("ProgramFiles(x86)").unwrap_or_default();
    let lad = env::var("LOCALAPPDATA").unwrap_or_default();
    let cands = [
        format!("{pf}\\Google\\Chrome\\Application\\chrome.exe"),
        format!("{pfx}\\Google\\Chrome\\Application\\chrome.exe"),
        format!("{lad}\\Google\\Chrome\\Application\\chrome.exe"),
        format!("{pfx}\\Microsoft\\Edge\\Application\\msedge.exe"),
        format!("{pf}\\Microsoft\\Edge\\Application\\msedge.exe"),
    ];
    cands.iter().map(PathBuf::from).find(|p| p.exists())
}

fn abrir(c: &Criadora) -> Result<(), String> {
    let nav = achar_navegador().ok_or("Não achei o Chrome nem o Edge instalado.")?;
    let sl = slug(&c.nome);
    let dd = data_dir();
    let primeira_vez = !dd.join(&sl).exists();

    let mut cmd = Command::new(nav);
    cmd.arg(format!("--user-data-dir={}", dd.display()))
        .arg(format!("--profile-directory={sl}"))
        .arg("--no-first-run")
        .arg("--no-default-browser-check");

    if primeira_vez {
        let abas = if c.abas.is_empty() { abas_padrao() } else { c.abas.clone() };
        for a in &abas {
            cmd.arg(a);
        }
    } else {
        cmd.arg("--restore-last-session");
    }

    cmd.spawn().map_err(|e| e.to_string())?;
    Ok(())
}

// ---------- app ----------
struct App {
    lista: Vec<Criadora>,
    novo: String,
    erro: String,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(CREME);
        visuals.panel_fill = PRETO;
        visuals.window_fill = PRETO;
        cc.egui_ctx.set_visuals(visuals);
        Self {
            lista: carregar(),
            novo: String::new(),
            erro: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut abrir_idx: Option<usize> = None;
        let mut remover_idx: Option<usize> = None;
        let mut adicionar = false;

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(PRETO).inner_margin(egui::Margin::symmetric(22.0, 20.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("girl").size(30.0).strong().color(CREME));
                    ui.add_space(-10.0);
                    ui.label(egui::RichText::new(".").size(30.0).strong().color(ROSA));
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("PAINEL")
                            .size(13.0)
                            .strong()
                            .color(CREME.gamma_multiply(0.5)),
                    );
                });
                ui.label(
                    egui::RichText::new("Um navegador separado pra cada criadora. Clique em Abrir.")
                        .size(13.0)
                        .color(CREME.gamma_multiply(0.6)),
                );
                ui.add_space(16.0);

                ui.horizontal_wrapped(|ui| {
                    for (i, c) in self.lista.iter().enumerate() {
                        egui::Frame::none()
                            .fill(CARD)
                            .rounding(10.0)
                            .inner_margin(egui::Margin::same(12.0))
                            .stroke(egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(42, 42, 42)))
                            .show(ui, |ui| {
                                ui.set_width(146.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new(&c.nome).size(15.0).strong().color(CREME),
                                    );
                                    ui.add_space(8.0);
                                    ui.horizontal(|ui| {
                                        let b = egui::Button::new(
                                            egui::RichText::new("Abrir").color(CREME).strong(),
                                        )
                                        .fill(ROSA)
                                        .rounding(7.0);
                                        if ui.add(b).clicked() {
                                            abrir_idx = Some(i);
                                        }
                                        if ui
                                            .add(egui::Button::new("✕").rounding(7.0))
                                            .on_hover_text("Tirar do painel")
                                            .clicked()
                                        {
                                            remover_idx = Some(i);
                                        }
                                    });
                                });
                            });
                        ui.add_space(6.0);
                    }
                });

                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut self.novo)
                            .hint_text("Nome da criadora")
                            .desired_width(200.0),
                    );
                    let clicou = ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Adicionar").color(CREME).strong(),
                            )
                            .fill(ROSA)
                            .rounding(7.0),
                        )
                        .clicked();
                    let enter = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    if clicou || enter {
                        adicionar = true;
                    }
                });

                if !self.erro.is_empty() {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(&self.erro).size(12.0).color(egui::Color32::from_rgb(255, 107, 157)));
                }

                ui.add_space(14.0);
                ui.label(
                    egui::RichText::new(
                        "Cada perfil guarda seus próprios logins e abas, isolado do seu Chrome.",
                    )
                    .size(11.0)
                    .color(CREME.gamma_multiply(0.4)),
                );
            });

        // aplica as ações depois da UI
        if let Some(i) = abrir_idx {
            self.erro.clear();
            if let Err(e) = abrir(&self.lista[i]) {
                self.erro = e;
            }
        }
        if let Some(i) = remover_idx {
            self.lista.remove(i);
            salvar(&self.lista);
        }
        if adicionar {
            let nome = self.novo.trim().to_string();
            if !nome.is_empty() {
                self.lista.push(Criadora {
                    nome,
                    abas: abas_padrao(),
                });
                salvar(&self.lista);
                self.novo.clear();
            }
        }
    }
}

fn load_icon() -> egui::IconData {
    let bytes = include_bytes!("../assets/icon.png");
    match image::load_from_memory(bytes) {
        Ok(img) => {
            let img = img.to_rgba8();
            let (w, h) = img.dimensions();
            egui::IconData {
                rgba: img.into_raw(),
                width: w,
                height: h,
            }
        }
        Err(_) => egui::IconData {
            rgba: vec![0, 0, 0, 0],
            width: 1,
            height: 1,
        },
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([560.0, 540.0])
            .with_min_inner_size([420.0, 320.0])
            .with_icon(Arc::new(load_icon())),
        ..Default::default()
    };
    eframe::run_native(
        "Girl · Painel",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

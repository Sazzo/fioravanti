use dialoguer::{theme::ColorfulTheme, Select};

pub fn prompt_select_sfx(sfx_list: &[String]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Selecione um som para tocar")
        .items(&sfx_list[..])
        .interact()
        .unwrap()
}

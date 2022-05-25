use crate::yml_definitions::*;
use handlebars::Handlebars;
use std::fs::read_to_string;

fn read_md(filename: &str) -> Podcast {
    let file_content = &read_to_string(filename).expect("Can't read the RSS file");
    let yaml = &file_content[0..file_content.len() - 5];
    serde_yaml::from_str(yaml).expect("File with wrong format")
}

fn markdown_to_xml(podcast: Podcast) -> String {
    let handlebars = Handlebars::new();
    let template = String::from_utf8_lossy(include_bytes!("../xml_template.handlebars"));
    handlebars
        .render_template(&template, &podcast)
        .expect("Couldn't render the template")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_md() {
        assert_eq!(
            read_md("examples/example_input.md"),
            Podcast {
                title: String::from("Ni cero, ni uno"),
                description: String::from("Un punto de vista diferente, peculiar y atrevido sobre la industria del desarrollo de software y sobre las habilidades que más se necesitan en este mundo tecnológico que en verdad gira en torno a las personas.\n\nCarlos Blé, fundador de varias empresas y actual director de Lean Mind, narra sus experiencias y cuenta con colaboraciones de profesionales de diversos ámbitos."),
                author_name: String::from("Carlos Blé"),
                author_email: String::from("carlos@carlosble.com"),
                episodes: vec![Episode {
                    title: String::from("E32: ¿Para qué impartir una ponencia?"),
                    link: String::from("https://podcast.carlosble.com/podcast/e32-para-que-impartir-una-ponencia/"),
                    date: String::from("Thu, 14 Apr 2022 15:58:24 +0000"),
                    description: String::from("¿Para qué dar una charla?, ¿por qué exponerse?, ¿qué puedes aportar cuando parece que ya está todo en internet?. En este episodio converso con Adrián Ferrera, Francisco Mesa y Miguel Cabrera. Además, contamos con mensajes grabados de ponentes que me encantan, Yodra López, Jorge J. Barroso, Aida Albarrán y Alexandra Rivero. Contribuciones: Yodra López (1:07:09)..."),
                    link_mp3: String::from("https://podcast.carlosble.com/podcast-download/1220/e32-para-que-impartir-una-ponencia.mp3"),
                }, Episode {
                    title: String::from("E31: El sendero de la maestría (III)"),
                    link: String::from("https://podcast.carlosble.com/podcast/e31-el-sendero-de-la-maestria-iii/"),
                    date: String::from("Fri, 18 Mar 2022 09:43:27 +0000"),
                    description: String::from("Terminamos la serie de epidosodios sobre aprendizaje (por ahora), con el gran Jose Enrique Rodríguez Huerta, actualmente director de Codurance España. En esta conversación José nos habla de cultura empresarial, liderazgo, carrera profesional, trabajo en equipo y por supuesto, aprendizaje. Recursos citados en el episodio (y otros que olvidé citar): Libro: Tribal Leadership Trabajar en..."),
                    link_mp3: String::from("https://podcast.carlosble.com/podcast-download/1216/e31-el-sendero-de-la-maestria-iii.mp3"),
                }],
            }
        );
    }

    #[test]
    fn generate_xml() -> std::io::Result<()> {
        let expected = read_to_string("examples/example_output.xml")?;
        let actual = markdown_to_xml(read_md("examples/example_input.md"));
        actual
            .split("\n")
            .zip(expected.split("\n"))
            .enumerate()
            .for_each(|(line_number, (actual_line, expected_line))| {
                assert_eq!(actual_line, expected_line, "\nDifference in line {line_number}:\nactual:\t{actual_line}\nexpected:\t{expected_line} ")
            });
        Ok(())
    }
}

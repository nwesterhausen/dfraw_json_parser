use specta::ts::{export, BigIntExportBehavior, ExportConfiguration};

fn header(struct_path: &str) -> String {
    format!("// File generated by specta. Do not edit!\n//\n/// lib/{struct_path}\n\n")
}

struct TypescriptBinding {
    output_path: std::path::PathBuf,
}

impl TypescriptBinding {
    fn new(output_path: std::path::PathBuf) -> Self {
        Self { output_path }
    }

    fn write(&self, struct_path: &str, bindings_filename: &str, imports: Option<String>, x: &str) {
        match std::fs::write(
            self.output_path
                .join(bindings_filename)
                .as_os_str()
                .to_str()
                .unwrap_or_default(),
            format!(
                "{}\n{}\n{}",
                header(struct_path),
                imports.unwrap_or_default(),
                &x
            ),
        ) {
            Err(e) => {
                eprintln!("{e:?}");
                eprintln!("Failed to write bindings for {struct_path} to {bindings_filename}");
            }
            Ok(()) => {
                println!("Wrote bindings for {struct_path} to {bindings_filename}");
            }
        }
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn generate_ts_bindings() {
    // get our current working directory
    let cwd = std::env::current_dir().unwrap();
    // set lib/bindings/AllBindings.d.ts as the output file
    let output_dir = cwd.join("bindings");
    // make sure output dir exists
    std::fs::create_dir_all(&output_dir).unwrap();
    eprintln!("Output dir: {:?}", &output_dir);
    let ts_bindings = TypescriptBinding::new(output_dir);

    let config = ExportConfiguration::default()
        .export_by_default(Some(true))
        .bigint(BigIntExportBehavior::String);

    let bindings: Vec<String> = vec![
        match export::<dfraw_json_parser::biome::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export biome::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature::Creature>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature::Creature");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_caste::Gait>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_caste::Gait");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_caste::GaitType>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_caste::GaitType");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_caste::GaitModifier>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_caste::GaitModifier");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_caste::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_caste::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_caste::Caste>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_caste::Caste");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_effect::PropertyToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_effect::PropertyToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_effect::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_effect::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_variation::CreatureVariation>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_variation::CreatureVariation");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_variation::Rule>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_variation::Rule");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::creature_variation::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export creature_variation::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::entity::Entity>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export entity::Entity");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::entity::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export entity::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::Graphic>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::Graphic");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::SpriteGraphic>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::SpriteGraphic");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::SpriteLayer>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::SpriteLayer");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::TilePage>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::TilePage");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::ColorModificationToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::ColorModificationToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::ConditionToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::ConditionToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::GraphicTypeToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::GraphicTypeToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::GrowthToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::GrowthToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::PlantGraphicTemplateToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::PlantGraphicTemplateToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::graphics::TilePageToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export graphics::TilePageToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::inorganic::Inorganic>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export inorganic::Inorganic");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::inorganic::EnvironmentClassToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export inorganic::EnvironmentClassToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::inorganic::InclusionTypeToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export inorganic::InclusionTypeToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::inorganic::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export inorganic::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::material::Material>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export material::Material");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::material::FuelTypeToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export material::FuelTypeToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::material::PropertyToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export material::PropertyToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::material::StateToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export material::StateToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::material::TypeToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export material::TypeToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::material::UsageToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export material::UsageToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::material_template::MaterialTemplate>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export material_template::MaterialTemplate");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::plant::Plant>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export plant::Plant");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::plant::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export plant::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::plant_growth::PlantGrowth>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export plant_growth::PlantGrowth");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::plant_growth::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export plant_growth::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::plant_growth::TypeToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export plant_growth::TypeTokenn");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::plant_growth::PlantPartToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export plant_growth::PlantPartToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::position::Position>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export position::Position");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::position::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export position::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::seed_material::SeedMaterial>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export seed_material::SeedMaterial");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::select_creature::SelectCreature>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export select_creature::SelectCreature");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::select_creature::SelectRules>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export select_creature::SelectRules");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::shrub::Shrub>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export shrub::Shru");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::shrub::SeasonToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export shrub::SeasonToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::shrub::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export shrub::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::syndrome::Syndrome>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export syndrome::Syndrome");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::syndrome::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export syndrome::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::tree::Tree>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export tree::Tree");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::tree::Token>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export tree::Token");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::tree::TwigPlacementToken>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export tree::TwigPlacementToken");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::unprocessed_raw::UnprocessedRaw>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export unprocessed_raw::UnprocessedRaw");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::unprocessed_raw::Modification>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export unprocessed_raw::Modification");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::MaterialMechanics>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::MaterialMechanics");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::RawMetadata>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::RawMetadata");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::ModuleInfoFile>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::ModuleInfoFile");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::SteamData>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::SteamData");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::StateName>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::StateName");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::MechanicalProperties>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::MechanicalProperties");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::Name>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::Name");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::SingPlurName>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::SingPlurName");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::ObjectType>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::ObjectType");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::RawModuleLocation>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::RawModuleLocation");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::BodySize>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::BodySize");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::Color>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::Color");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::Milkable>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::Milkable");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::Temperatures>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::Temperatures");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::Tile>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::Tile");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::graphics::Dimensions>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::graphics::Dimensions");
                eprintln!("{e:?}");
                String::new()
            }
        },
        match export::<dfraw_json_parser::parser::graphics::CustomGraphicExtension>(&config) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to export parser::graphics::CustomGraphicExtension");
                eprintln!("{e:?}");
                String::new()
            }
        },
    ];

    let missed = bindings.iter().filter(|x| x.is_empty()).collect::<Vec<_>>();
    // change missed into the number of missed bindings
    let missed = missed.len();
    // if there are missed bindings, print a warning
    if missed > 0 {
        eprintln!("Missed {missed} bindings");
    }

    // write the bindings to the output file
    ts_bindings.write(
        "dfraw_json_parser library",
        "DFRawJson.d.ts",
        None,
        &bindings.join("\n\n"),
    );
}

use std::collections::HashMap;

use crate::equipment::{create_blueprint, Blueprint};

/// Get the info on hero equipment (e.g. atk, def, etc.) from the Blueprints tab of the Official ST Sheet
pub fn _get_hero_equipment_data(path: String) -> HashMap<String, Blueprint> {
    let mut bp_map: HashMap<String, Blueprint> = Default::default();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_path(path)
        .unwrap();
    for result in reader.records() {
        let record = result.unwrap();

        bp_map.insert(
            record[0].to_string(),
            create_blueprint(
                record[0].to_string(),
                record[1].to_string(),
                record[2].to_string(),
                record[3].to_string().parse::<u16>().unwrap_or_default(),
                record[4].to_string().parse::<u16>().unwrap_or_default(),
                record[5].to_string().parse::<u8>().unwrap_or_default(),
                record[6].to_string().parse::<u32>().unwrap_or_default(),
                record[7].to_string().parse::<u32>().unwrap_or_default(),
                record[8].to_string(),
                record[9].to_string().parse::<f64>().unwrap_or_default(),
                record[10].to_string().parse::<u32>().unwrap_or_default(),
                record[11].to_string().parse::<f64>().unwrap_or_default(),
                record[12].to_string().parse::<u32>().unwrap_or_default(),
                record[13].to_string().parse::<u32>().unwrap_or_default(),
                record[14].to_string().parse::<u32>().unwrap_or_default(),
                record[15].to_string().parse::<u32>().unwrap_or_default(),
                // 16 blank
                record[17].to_string(),
                record[18].to_string().parse::<u8>().unwrap_or_default(),
                record[19].to_string(),
                record[20].to_string().parse::<u8>().unwrap_or_default(),
                record[21].to_string(),
                record[22].to_string().parse::<u8>().unwrap_or_default(),
                // 23 blank
                record[24].to_string().parse::<u16>().unwrap_or_default(),
                record[25].to_string().parse::<u16>().unwrap_or_default(),
                record[26].to_string().parse::<u16>().unwrap_or_default(),
                record[27].to_string().parse::<u16>().unwrap_or_default(),
                record[28].to_string().parse::<u16>().unwrap_or_default(),
                record[29].to_string().parse::<u16>().unwrap_or_default(),
                record[30].to_string().parse::<u16>().unwrap_or_default(),
                record[31].to_string().parse::<u16>().unwrap_or_default(),
                record[32].to_string().parse::<u16>().unwrap_or_default(),
                record[33].to_string().parse::<u16>().unwrap_or_default(),
                // 34 blank
                record[35].to_string(),
                record[36].to_string(),
                record[37].to_string().parse::<u8>().unwrap_or_default(),
                record[38].to_string(),
                record[39].to_string(),
                record[40].to_string().parse::<u8>().unwrap_or_default(),
                // 41 blank
                record[42].to_string().parse::<f64>().unwrap_or_default(),
                record[43].to_string().parse::<f64>().unwrap_or_default(),
                record[44].to_string().parse::<f64>().unwrap_or_default(),
                record[45].to_string().parse::<f64>().unwrap_or_default(),
                record[46].to_string().parse::<f64>().unwrap_or_default(),
                // 47 blank
                record[48].to_string(),
                record[49].to_string(),
                // 50 blank
                // 51-60: crafting upgrades
                // 61 blank
                // 61-67: ascension upgrades
                // 68 blank
                record[69].to_string().parse::<u16>().unwrap_or_default(),
                record[70].to_string().parse::<u16>().unwrap_or_default(),
                record[71].to_string().parse::<u16>().unwrap_or_default(),
                record[72].to_string().parse::<u16>().unwrap_or_default(),
            ),
        );
    }

    return bp_map;
}

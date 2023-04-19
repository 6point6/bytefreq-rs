use chrono::{NaiveDateTime, Utc};
use chrono::DateTime;

const DELIMITER: &str = "\t";

fn source_collection_identifier(source_collection_identifier: i32) -> String {
    match source_collection_identifier {
        1 => String::from("WEB"),
        2 => String::from("CITATION_ONLY"),
        3 => String::from("CORE"),
        4 => String::from("DTIC"),
        5 => String::from("JSTOR"),
        6 => String::from("NON_TEXTUAL_SOURCE"),
        _ => String::from("UNKNOWN"),
    }
}

fn parse_gkg_v2(str: &str) -> GKGEventV2 {
    let values = str.split(DELIMITER).collect::<Vec<&str>>();
            GKGEventV2 {
                gkg_record_id: build_gkg_record_id(values[0]),
                publish_date: build_publish_date(values[1]),
                source_collection_identifier: Some(build_source_collection_identifier(values[2])),
                source_common_name: Option::from(values[3].to_string()),
                document_identifier: Option::from(values[4].to_string()),
                counts: build_counts(values[5]),
                enhanced_counts: build_enhanced_counts(values[6]),
                themes: build_themes(values[7]),
                enhanced_themes: build_enhanced_themes(values[8]),
                locations: build_locations(values[9]),
                enhanced_locations: build_enhanced_locations(values[10]),
                persons: build_persons(values[11]),
                enhanced_persons: build_enhanced_persons(values[12]),
                organisations: build_organisations(values[13]),
                enhanced_organisations: build_enhanced_organisations(values[14]),
                // tone: build_tone(values[15]),
                enhanced_dates: build_enhanced_dates(values[16]),
                // gcams: build_gcams(values[17]),
                sharing_image: Option::from(values[18].to_string()),
                related_images: build_related_images(values[19]),
                social_image_embeds: build_social_image_embeds(values[20]),
                social_video_embeds: build_social_video_embeds(values[21]),
                quotations: build_quotations(values[22]),
                all_names: build_names(values[23]),
                amounts: build_amounts(values[24]),
                translation_info: Option::from(build_translation_info(values[25])),
                extras_xml: Option::from(values[26].to_string()),
                errors: Option::from("".to_string()),
            }
}

fn build_publish_date(str: &str) -> Option<DateTime<Utc>> {
    let date = NaiveDateTime::parse_from_str(str, "%Y%m%d%H%M%S");
    match date {
        Ok(dt) => Some(DateTime::<Utc>::from_utc(dt, Utc)),
        _ => None,
    }
}

fn build_related_images(str: &str) -> Vec<String> {
    str.split(";").map(|s| s.to_string()).collect()
}

// fn build_tone(str: &str) -> Option<Tone> {
//     let values: Vec<&str> = str.split(',').collect();
//     Some(Tone {
//         tone: values.get(0),
//         positive_score: values.get(1),
//         negative_score: values.get(2),
//         polarity: values.get(3),
//         activity_reference_density: values.get(4),
//         self_group_reference_density: values.get(5),
//         word_count: values.get(6),
//     })
// }

fn build_enhanced_dates(str: &str) -> Vec<EnhancedDate> {
    str.split(";")
        .filter_map(build_enhanced_date)
        .collect()
}

// fn build_gcams(str: &str) -> Vec<Gcam> {
//     str.split(',')
//         .filter_map(|s| build_gcam(s))
//         .collect()
// }

// fn build_gcam(str: &str) -> Option<Gcam> {
//     let split: Vec<&str> = str.split(':').collect();
//     let gcam_code = Some(split.get(0).unwrap_or(&"").to_string());
//     let gcam_value = Some(split.get(1));
//     Some(Gcam { gcam_code, gcam_value })
// }


fn build_enhanced_date(str: &str) -> Option<EnhancedDate> {
    let values: Vec<&str> = str.split("#").collect();
    if values.len() < 5 {
        return None; // or another value or error handling
    }
    let date_resolution = Some(values[0].parse::<i32>().unwrap_or(0));
    let month = Some(values[1].parse::<i32>().unwrap_or(0));
    let day = Some(values[2].parse::<i32>().unwrap_or(0));
    let year = Some(values[3].parse::<i32>().unwrap_or(0));
    let char_offset = Some(values[4].parse::<i32>().unwrap_or(0));
    Some(EnhancedDate {
        date_resolution,
        month,
        day,
        year,
        char_offset,
    })
}




fn build_gkg_record_id(s: &str) -> Option<GkgRecordId> {
    {
        let split = s.split('-').collect::<Vec<&str>>();
        let is_translingual =  split[1].contains('T');
        let number_in_batch = if let true = is_translingual {
             split[1].replace('T', "").parse::<i32>()
        } else {
             split[1].parse::<i32>()
        };
        Some(GkgRecordId {
            translingual: Option::from(is_translingual),
            number_in_batch: Option::from(number_in_batch.ok()?),
        })
    }
}

fn build_translation_info(s: &str) -> TranslationInfo {
    let values = s.split(';').collect::<Vec<&str>>();
    {
        TranslationInfo {
            srclc: Option::from(values[0].to_string()),
            eng: Option::from(values[1].to_string()),
        }
    }
}

fn build_amounts(s: &str) -> Vec<Amount> {
    s.split(';')
        .map(|amount| build_amount(amount))
        .filter(|amount| amount.is_some())
        .map(|amount| amount.unwrap())
        .collect()
}

fn build_amount(s: &str) -> Option<Amount> {
    let values = s.split(',').collect::<Vec<&str>>();
     Some({
        Amount {
            amount:  Some(values[0].parse::<f64>().ok()?),
            amount_type: Some(values[1].to_string()),
            char_offset: Some(values[2].parse::<i32>().ok()?)
        }
    })
}

fn build_names(s: &str) -> Vec<Name> {
    s.split(';')
        .map(|name| build_name(name))
        .filter(|name| name.is_some())
        .map(|name| name.unwrap())
        .collect()
}

fn build_name(s: &str) -> Option<Name> {
    let values = s.split(',').collect::<Vec<&str>>();
    if !values.len() > 0 {
        None
    } else {
        Some(Name {
            name: Option::from(values[0].to_string()),
            char_offset: Option::from(values[1].parse::<i32>().ok()?),
        })
    }
}

fn build_quotations(s: &str) -> Vec<Quotation> {
    s.split('#')
        .map(|quotation| build_quotation(quotation))
        .filter(|quotation| quotation.is_some())
        .map(|quotation| quotation.unwrap())
        .collect()
}

fn build_quotation(s: &str) -> Option<Quotation> {
    let values = s.split('|').collect::<Vec<&str>>();
    Some({
        Quotation {
            char_offset:  Some(values[0].parse::<i32>().ok()?),
            char_length: Some(values[1].parse::<i32>().ok()?),
            verb:  Some(values[2].to_string()),
            quote:  Some(values[3].to_string()),
        }
    })
}

fn build_social_image_embeds(s: &str) -> Vec<String> {
    s.split(';').map(|v| v.to_string()).collect()
}

fn build_social_video_embeds(s: &str) -> Vec<String> {
    s.split(';').map(|v| v.to_string()).collect()
}

fn build_organisations(str: &str) -> Vec<String> {
    str.split(";").map(|s| s.to_string()).collect()
}

fn build_enhanced_organisation(str: &str) -> Option<EnhancedOrganisation> {
    let blocks: Vec<&str> = str.split(",").collect();
    if blocks.len() > 1 {
        Some(EnhancedOrganisation {
            organisation: Some(blocks[0].to_owned()),
            char_offset:  blocks[1].parse::<i32>().ok(),
        })
    } else {
       None
    }

}

fn build_enhanced_organisations(str: &str) -> Vec<EnhancedOrganisation> {
    str.split(";")
        .filter_map(|s| build_enhanced_organisation(s))
        .collect()
}

fn build_source_collection_identifier(str: &str) -> String {
    source_collection_identifier(str.parse().unwrap_or(-1))
}

fn build_enhanced_locations(str: &str) -> Vec<EnhancedLocation> {
    str.split(";")
        .filter_map(|s| build_enhanced_location(s))
        .collect()
}

fn build_enhanced_location(str: &str) -> Option<EnhancedLocation> {
    let blocks: Vec<&str> = str.split('#').collect();
    if !blocks.len() > 7 {
        None
    } else {
        let geo_point = GeoPoint {
            latitude: Option::from(blocks[5].parse::<f32>().unwrap()),
            longitude: Option::from(blocks[6].parse::<f32>().unwrap()),
        };
        let location = Location {
            geo_type: geo_type(blocks[0].parse::<i32>().unwrap()),
            geo_name: Option::from(blocks[1].to_string()),
            country_code: Option::from(blocks[2].to_string()),
            adm1_code: Option::from(blocks[3].to_string()),
            adm2_code: Option::from(blocks[4].to_string()),
            geo_point: Some(geo_point),
            feature_id: Option::from(blocks[7].to_string()),
        };
        Some(EnhancedLocation {
            location: Some(location),
            char_offset: Option::from(blocks[8].parse::<i32>().unwrap()),
        })
    }
}

fn build_enhanced_persons(str: &str) -> Vec<EnhancedPerson> {
    str.split(',')
        .map(|s|  EnhancedPerson {
            person: Some(s.to_string()),
            char_offset: s.parse::<i32>().ok(),
        })
        .collect()
}

fn geo_type(geo_type: i32) -> Option<String> {
    Some(match geo_type {
        1 => String::from("COUNTRY"),
        2 => String::from("USSTATE"),
        3 => String::from("USCITY"),
        4 => String::from("WORLDCITY"),
        5 => String::from("WORLDSTATE"),
        _ => String::from("UNKNOWN"),
    })
}


fn build_persons(str: &str) -> Vec<String> {
    str.split(';').map(|s| s.to_string()).collect()
}



fn build_locations(str: &str) -> Vec<Location> {
    str.split(';')
        .map(|x| build_location(x))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}
fn build_location(str: &str) -> Option<Location> {
    let blocks = str.split("#").collect::<Vec<&str>>();
    if !blocks.len() > 6 {
        None
    } else {
        let geo_point = GeoPoint {
            latitude: Option::from(blocks[4].parse::<f32>().unwrap()),
            longitude: Option::from(blocks[5].parse::<f32>().unwrap()),
        };
        Some(Location {
            geo_type: geo_type(blocks[0].parse::<i32>().unwrap()),
            geo_name: Option::from(blocks[1].to_string()),
            country_code: Option::from(blocks[2].to_string()),
            adm1_code: Option::from(blocks[3].to_string()),
            adm2_code: None,
            geo_point: Some( geo_point),
            feature_id: Option::from(blocks[6].to_string()),
        })
    }
}

fn build_enhanced_themes(str: &str) -> Vec<EnhancedTheme> {
    str.split(";")
        .map(|s| build_enhanced_theme(s))
        .filter(|t| t.is_some())
        .map(|t| t.unwrap())
        .collect::<Vec<EnhancedTheme>>()
}

fn build_enhanced_theme(str: &str) -> Option<EnhancedTheme> {
    let blocks = str.split(",").collect::<Vec<&str>>();
    if blocks.len() > 1 {
        Some( EnhancedTheme {
            theme: Option::from(blocks[0].to_string()),
            char_offset: Some(blocks[1].parse::<i32>().unwrap()),
        })
    } else {
        None
    }

}

fn build_themes(str: &str) -> Vec<String> {
    str.split(";").map(|s| s.to_string()).collect::<Vec<String>>()
}

fn build_enhanced_counts(str: &str) -> Vec<EnhancedCount> {
    str.split(";")
        .map(|s| build_enhanced_count(s))
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect::<Vec<EnhancedCount>>()
}

fn build_enhanced_count(str: &str) -> Option<EnhancedCount> {
     {
        let count =  build_count(str).unwrap();
        Some(EnhancedCount {
            count: Option::from(count),
            char_offset: str.rsplit("#").next().unwrap().parse::<i32>().ok(),
        })
    }
}

fn build_count(str: &str) -> Option<Count> {
    let blocks = str.split("#").collect::<Vec<&str>>();
    if blocks.len() > 9 {
        let geo_point = GeoPoint {
            latitude: Option::from(blocks[7].parse::<f32>().unwrap_or(0.0)),
            longitude: Option::from(blocks[8].parse::<f32>().unwrap_or(0.0)),
        };
        let location = Location {
            geo_type: geo_type(blocks[3].parse::<i32>().unwrap()),
            geo_name: Option::from(blocks[4].to_string()),
            country_code: Option::from(blocks[5].to_string()),
            adm1_code: Option::from(blocks[6].to_string()),
            adm2_code: None,
            geo_point: Some(geo_point),
            feature_id: Option::from(blocks[9].to_string()),
        };
        Some(Count {
            count_type: Option::from(blocks[0].to_string()),
            count: Option::from(blocks[1].parse::<i64>().unwrap()),
            object_type: Option::from(blocks[2].to_string()),
            location: Some(location),
        })

    } else {
        Some(Count {
        count_type: None,
        count: None,
        object_type: None,
        location: None,
            }
        )
    }
}

fn build_counts(str: &str) -> Vec<Count> {
    str.split(";")
        .filter_map(|s| build_count(s))
        .collect()
}

// #[derive(Debug)]
// struct Actor {
//     cameo_raw: Option<String>,
//     cameo_name: Option<String>,
//     cameo_country_code: Option<String>,
//     cameo_group_code: Option<String>,
//     cameo_ethnic_code: Option<String>,
//     cameo_religion1_code: Option<String>,
//     cameo_religion2_code: Option<String>,
//     cameo_type1_code: Option<String>,
//     cameo_type2_code: Option<String>,
//     cameo_type3_code: Option<String>,
// }

#[derive(Debug)]
struct Count {
    count_type: Option<String>,
    count: Option<i64>,
    object_type: Option<String>,
    location: Option<Location>,
}

// #[derive(Debug)]
// struct Tone {
//     tone: Option<String>,
//     positive_score: Option<String>,
//     negative_score: Option<String>,
//     polarity: Option<String>,
//     activity_reference_density: Option<String>,
//     self_group_reference_density: Option<String>,
//     word_count: Option<String>,
// }

#[derive(Debug)]
struct EnhancedLocation {
    location: Option<Location>,
    char_offset: Option<i32>,
}

#[derive(Debug)]
struct EnhancedTheme {
    theme: Option<String>,
    char_offset: Option<i32>,
}

#[derive(Debug)]
struct EnhancedPerson {
    person: Option<String>,
    char_offset: Option<i32>,
}

#[derive(Debug)]
struct EnhancedOrganisation {
    organisation: Option<String>,
    char_offset: Option<i32>,
}

// #[derive(Debug)]
// struct Gcam {
//     gcam_code: Option<String>,
//     gcam_value: Option<f64>,
// }

#[derive(Debug)]
struct GkgRecordId {
    translingual: Option<bool>,
    number_in_batch: Option<i32>,
}

#[derive(Debug)]
struct EnhancedDate {
    date_resolution: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
    year: Option<i32>,
    char_offset: Option<i32>,
}

#[derive(Debug)]
struct EnhancedCount {
    count: Option<Count>,
    char_offset: Option<i32>,
}

#[derive(Debug)]
struct Location {
    geo_type: Option<String>,
    geo_name: Option<String>,
    country_code: Option<String>,
    adm1_code: Option<String>,
    adm2_code: Option<String>,
    geo_point: Option<GeoPoint>,
    feature_id: Option<String>,
}

#[derive(Debug)]
struct GeoPoint {
    latitude: Option<f32>,
    longitude: Option<f32>,
}

#[derive(Debug)]
struct Name {
    name: Option<String>,
    char_offset: Option<i32>,
}

#[derive(Debug)]
struct GKGEventV2 {
    gkg_record_id: Option<GkgRecordId>,
    publish_date: Option<DateTime<Utc>>,
    source_collection_identifier: Option<String>,
    source_common_name: Option<String>,
    document_identifier: Option<String>,
    counts: Vec<Count>,
    enhanced_counts: Vec<EnhancedCount>,
    themes: Vec<String>,
    enhanced_themes: Vec<EnhancedTheme>,
    locations: Vec<Location>,
    enhanced_locations: Vec<EnhancedLocation>,
    persons: Vec<String>,
    enhanced_persons: Vec<EnhancedPerson>,
    organisations: Vec<String>,
    enhanced_organisations: Vec<EnhancedOrganisation>,
    // tone: Option<Tone>,
    enhanced_dates: Vec<EnhancedDate>,
    // gcams: Vec<Gcam>,
    sharing_image: Option<String>,
    related_images: Vec<String>,
    social_image_embeds: Vec<String>,
    social_video_embeds: Vec<String>,
    quotations: Vec<Quotation>,
    all_names: Vec<Name>,
    amounts: Vec<Amount>,
    translation_info: Option<TranslationInfo>,
    extras_xml: Option<String>,
    errors: Option<String>,
}

#[derive(Debug)]
struct TranslationInfo {
    srclc: Option<String>,
    eng: Option<String>,
}

#[derive(Debug)]
struct Amount {
    amount: Option<f64>,
    amount_type: Option<String>,
    char_offset: Option<i32>,
}

#[derive(Debug)]
struct Quotation {
    char_length: Option<i32>,
    verb: Option<String>,
    quote: Option<String>,
    char_offset: Option<i32>,
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use crate::parser::gdelt::gdelt_parser::{GKGEventV2, parse_gkg_v2};
    use encoding_rs::WINDOWS_1252;
    use encoding_rs_io::DecodeReaderBytesBuilder;


    #[test]
    fn test_reading_gkg_file() {
        let file = File::open("resources/20230417163000.translation.gkg.csv").unwrap();
        let reader = BufReader::new(
            DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1252))
                .build(file));

        for line in reader.lines() {
            let record_str = line.unwrap();
            let out: GKGEventV2 = parse_gkg_v2(&record_str);

            // process the record
            // println!("{:?}", out);
        }
    }

    #[test]
    fn test_gkg_line() {
        let line = "20230417163000-T0	20230417163000	1	diariandorra.ad	https://www.diariandorra.ad/noticies/nacional/2023/04/17/alliberen_gamarus_atrapat_interior_una_xemeneia_219392_1125.html			UNGP_FORESTS_RIVERS_OCEANS;	UNGP_FORESTS_RIVERS_OCEANS,385;	1#Andorra#AN#AN#42.5#1.5#AN	1#Andorra#AN#AN##42.5#1.5#AN#596					-1.01010101010101,3.03030303030303,4.04040404040404,7.07070707070707,23.2323232323232,0,89		wc:89,nwc:146,c1.3:1,c12.1:5,c12.10:4,c12.12:2,c12.14:2,c12.3:3,c12.4:1,c12.5:3,c12.7:5,c12.8:1,c12.9:5,c13.14:1,c14.1:4,c14.10:6,c14.11:7,c14.2:12,c14.3:7,c14.4:3,c14.5:15,c14.6:1,c14.7:1,c14.8:1,c14.9:1,c15.126:1,c15.130:1,c15.131:1,c15.175:1,c15.187:1,c15.227:1,c16.100:3,c16.105:1,c16.106:1,c16.109:1,c16.110:15,c16.114:4,c16.115:1,c16.116:3,c16.117:11,c16.118:6,c16.12:13,c16.120:6,c16.121:4,c16.125:10,c16.126:11,c16.127:6,c16.129:8,c16.130:4,c16.131:5,c16.134:7,c16.135:1,c16.138:4,c16.139:1,c16.140:7,c16.145:8,c16.146:6,c16.153:4,c16.157:5,c16.159:6,c16.16:5,c16.161:5,c16.162:1,c16.163:10,c16.164:2,c16.165:1,c16.19:5,c16.2:5,c16.22:3,c16.24:2,c16.26:19,c16.27:1,c16.3:3,c16.31:8,c16.32:3,c16.33:6,c16.35:4,c16.36:2,c16.37:8,c16.38:4,c16.4:15,c16.41:3,c16.45:1,c16.46:1,c16.47:12,c16.48:1,c16.50:2,c16.51:1,c16.52:7,c16.53:1,c16.54:1,c16.56:1,c16.57:49,c16.58:7,c16.6:11,c16.60:1,c16.62:1,c16.63:4,c16.64:2,c16.65:3,c16.66:6,c16.68:7,c16.69:1,c16.7:3,c16.70:6,c16.71:3,c16.75:1,c16.78:2,c16.82:1,c16.84:9,c16.87:12,c16.88:9,c16.89:3,c16.9:2,c16.90:2,c16.91:3,c16.92:9,c16.94:2,c16.95:7,c16.96:1,c16.97:2,c16.98:5,c16.99:2,c17.1:30,c17.10:6,c17.11:11,c17.12:3,c17.13:2,c17.14:1,c17.15:8,c17.16:10,c17.18:3,c17.19:4,c17.2:2,c17.22:2,c17.23:5,c17.24:6,c17.25:1,c17.27:7,c17.29:4,c17.3:2,c17.30:4,c17.31:9,c17.32:3,c17.33:8,c17.34:1,c17.35:1,c17.36:7,c17.37:4,c17.39:5,c17.4:20,c17.40:3,c17.41:8,c17.42:9,c17.43:7,c17.5:15,c17.7:19,c17.8:8,c17.9:7,c2.1:5,c2.101:5,c2.102:2,c2.104:14,c2.108:3,c2.11:3,c2.110:1,c2.111:1,c2.112:2,c2.113:2,c2.114:3,c2.115:3,c2.116:2,c2.119:21,c2.12:1,c2.121:5,c2.122:3,c2.123:1,c2.125:1,c2.126:2,c2.127:2,c2.128:2,c2.129:6,c2.132:1,c2.133:1,c2.137:3,c2.139:1,c2.14:11,c2.140:1,c2.141:1,c2.143:7,c2.144:1,c2.145:1,c2.147:12,c2.148:6,c2.15:6,c2.150:2,c2.152:1,c2.153:3,c2.154:6,c2.155:10,c2.156:6,c2.157:9,c2.158:4,c2.159:1,c2.160:5,c2.162:1,c2.166:1,c2.17:1,c2.172:2,c2.173:2,c2.177:2,c2.179:3,c2.18:1,c2.180:1,c2.181:1,c2.182:1,c2.183:2,c2.185:19,c2.187:7,c2.192:3,c2.193:8,c2.195:7,c2.196:2,c2.197:2,c2.198:13,c2.199:4,c2.203:3,c2.204:7,c2.205:2,c2.206:3,c2.207:2,c2.21:2,c2.210:9,c2.214:2,c2.217:2,c2.220:2,c2.225:1,c2.226:2,c2.23:6,c2.25:1,c2.26:5,c2.27:5,c2.28:2,c2.30:1,c2.31:2,c2.32:1,c2.33:3,c2.34:4,c2.35:2,c2.38:2,c2.39:17,c2.4:1,c2.42:1,c2.44:2,c2.45:2,c2.46:12,c2.47:1,c2.48:3,c2.50:1,c2.52:1,c2.54:2,c2.55:1,c2.58:1,c2.6:1,c2.62:1,c2.66:1,c2.68:1,c2.7:1,c2.70:1,c2.71:1,c2.73:1,c2.75:9,c2.76:60,c2.77:7,c2.78:12,c2.79:2,c2.80:8,c2.82:4,c2.83:2,c2.85:1,c2.86:4,c2.87:1,c2.88:1,c2.89:5,c2.9:3,c2.93:1,c2.95:15,c2.97:1,c2.98:5,c25.2:2,c3.1:6,c3.2:4,c35.12:1,c35.14:1,c35.15:3,c35.18:3,c35.2:1,c35.20:2,c35.31:5,c35.32:7,c35.33:2,c35.4:2,c35.5:1,c39.14:3,c39.17:1,c39.2:2,c39.3:5,c39.34:2,c39.36:3,c39.37:4,c39.38:2,c39.4:5,c39.40:3,c39.41:1,c39.5:2,c4.15:2,c4.23:1,c4.9:1,c40.5:1,c41.1:4,c42.1:26,c5.10:9,c5.11:3,c5.12:15,c5.16:1,c5.17:1,c5.23:2,c5.25:1,c5.27:1,c5.28:2,c5.29:1,c5.30:7,c5.31:2,c5.34:2,c5.35:2,c5.36:5,c5.40:3,c5.43:1,c5.46:11,c5.47:1,c5.48:2,c5.49:10,c5.5:2,c5.50:9,c5.51:9,c5.52:13,c5.53:12,c5.54:6,c5.6:1,c5.61:6,c5.62:40,c5.7:1,c5.9:5,c6.1:1,c6.2:2,c6.4:4,c6.5:1,c7.1:7,c7.2:2,c8.2:1,c8.23:4,c8.25:1,c8.38:2,c8.4:1,c8.41:1,c8.42:2,c8.43:5,c8.7:1,c9.1:2,c9.1012:1,c9.1016:1,c9.1030:1,c9.107:1,c9.109:2,c9.111:2,c9.113:2,c9.12:2,c9.124:2,c9.128:4,c9.14:1,c9.141:1,c9.145:2,c9.149:2,c9.151:3,c9.158:4,c9.160:1,c9.161:2,c9.162:2,c9.164:2,c9.166:1,c9.168:2,c9.170:2,c9.175:2,c9.177:1,c9.18:1,c9.182:1,c9.184:2,c9.192:1,c9.193:1,c9.194:1,c9.198:3,c9.199:2,c9.20:2,c9.200:2,c9.207:2,c9.212:2,c9.23:2,c9.231:2,c9.233:1,c9.235:3,c9.237:1,c9.245:1,c9.246:1,c9.249:3,c9.25:1,c9.250:2,c9.253:2,c9.260:1,c9.270:2,c9.288:1,c9.293:1,c9.294:1,c9.3:2,c9.30:2,c9.302:2,c9.308:3,c9.32:2,c9.326:1,c9.361:2,c9.370:3,c9.371:2,c9.377:1,c9.383:3,c9.390:1,c9.42:3,c9.424:3,c9.428:1,c9.430:1,c9.440:1,c9.451:2,c9.454:2,c9.46:2,c9.462:1,c9.465:3,c9.479:2,c9.480:2,c9.489:1,c9.498:2,c9.5:1,c9.501:1,c9.511:5,c9.512:3,c9.513:1,c9.517:1,c9.524:1,c9.53:1,c9.531:3,c9.534:1,c9.535:1,c9.55:2,c9.551:1,c9.560:1,c9.564:1,c9.567:1,c9.57:2,c9.570:2,c9.571:2,c9.575:2,c9.579:1,c9.588:1,c9.59:2,c9.601:1,c9.603:1,c9.616:2,c9.619:1,c9.625:1,c9.627:1,c9.629:1,c9.632:2,c9.635:1,c9.639:1,c9.642:2,c9.648:2,c9.650:2,c9.653:1,c9.655:1,c9.659:1,c9.660:1,c9.674:1,c9.677:2,c9.681:1,c9.683:1,c9.686:2,c9.688:2,c9.692:1,c9.693:1,c9.696:1,c9.697:1,c9.698:2,c9.7:1,c9.701:2,c9.702:4,c9.704:2,c9.71:2,c9.710:1,c9.723:1,c9.724:3,c9.726:1,c9.727:3,c9.730:1,c9.731:1,c9.735:1,c9.740:1,c9.748:2,c9.75:2,c9.757:2,c9.76:1,c9.762:1,c9.766:1,c9.767:4,c9.770:1,c9.774:2,c9.780:1,c9.790:2,c9.792:1,c9.793:2,c9.795:2,c9.799:1,c9.802:2,c9.806:2,c9.808:1,c9.809:1,c9.812:3,c9.814:3,c9.816:1,c9.822:1,c9.83:4,c9.834:1,c9.838:1,c9.84:1,c9.841:1,c9.842:3,c9.858:1,c9.860:2,c9.863:3,c9.864:1,c9.865:1,c9.867:1,c9.868:1,c9.87:3,c9.882:3,c9.883:2,c9.884:3,c9.9:3,c9.902:1,c9.903:1,c9.909:1,c9.920:1,c9.930:1,c9.931:1,c9.935:2,c9.942:3,c9.948:1,c9.953:1,c9.956:1,c9.957:1,c9.958:1,c9.964:1,c9.966:1,c9.967:3,c9.968:2,c9.972:3,c9.973:3,c9.978:2,c9.980:1,c9.984:2,c9.997:1,v10.1:0.193619791666667,v10.2:0.333017676767677,v11.1:-0.0098681,v19.1:5.73909090909091,v19.2:4.76090909090909,v19.3:5.25545454545455,v19.4:5.70727272727273,v19.5:4.88181818181818,v19.6:5.30454545454545,v19.7:5.74090909090909,v19.8:4.71,v19.9:5.17636363636364,v20.11:0.5,v20.13:0.371666666666667,v20.15:0.319555555555556,v20.16:-0.25,v21.1:5.22857142857143,v26.1:-0.65,v42.10:-0.156110275615385,v42.11:-0.143771992538462,v42.2:0.139126366346154,v42.3:0.0987137305769231,v42.4:0.0825610909615385,v42.5:0.0795910580769231,v42.6:0.0974178378846154,v42.7:-0.191424707153846,v42.8:-0.153364071653846,v42.9:-0.151256162,c23.13:1,c23.15:2,v23.13:0.271,v23.15:0.2605	https://cdn01.diariandorra.ad/uploads/imagenes/bajacalidad/2023/04/17/_00banders_859573f4.jpg?a5335b4ed2ee43fab4676aada5009089		https://pic.twitter.com/Hi5ObRsYLW;			Thankfully East,546		srclc:cat;eng:GT-CAT 1.0	<PAGE_LINKS>https://t.co/Hi5ObRsYLW;https://twitter.com/BandersAndorra/status/1647987299903315968;https://twitter.com/hashtag/carabo;https://twitter.com/hashtag/chouettehulotte;https://twitter.com/hashtag/collaboraci%C3%B3ciutadana;https://twitter.com/hashtag/gamarus;https://twitter.com/hashtag/mussol</PAGE_LINKS><PAGE_AUTHORS>Redacci&#xF3;;Andorra la Vella</PAGE_AUTHORS><PAGE_TITLE>Alliberen un gamar&#xFA;s atrapat a l'interior d'una xemeneia</PAGE_TITLE>";
        let out: GKGEventV2 = parse_gkg_v2(&line);

        // themes
        assert_eq!(out.themes[0],"UNGP_FORESTS_RIVERS_OCEANS");

        // source_common_name
        assert_eq!(out.source_common_name.unwrap(),"diariandorra.ad");

        // translation info
        assert_eq!( out.translation_info.as_ref().unwrap().eng.clone().unwrap(), "eng:GT-CAT 1.0");
        assert_eq!( out.translation_info.as_ref().unwrap().srclc.clone().unwrap(), "srclc:cat");

        //publish_date
        assert_eq!( out.publish_date.unwrap().to_string(), "2023-04-17 16:30:00 UTC");
    }

}



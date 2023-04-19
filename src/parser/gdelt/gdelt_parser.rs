use chrono::format::Numeric::Timestamp;
use chrono::{NaiveDateTime, Utc};
use std::num::ParseIntError;
use std::str::FromStr;
use chrono::DateTime;
use chrono::format::strftime::StrftimeItems;
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

#[derive(Debug)]
struct Actor {
    cameo_raw: Option<String>,
    cameo_name: Option<String>,
    cameo_country_code: Option<String>,
    cameo_group_code: Option<String>,
    cameo_ethnic_code: Option<String>,
    cameo_religion1_code: Option<String>,
    cameo_religion2_code: Option<String>,
    cameo_type1_code: Option<String>,
    cameo_type2_code: Option<String>,
    cameo_type3_code: Option<String>,
}

#[derive(Debug)]
struct Count {
    count_type: Option<String>,
    count: Option<i64>,
    object_type: Option<String>,
    location: Option<Location>,
}

#[derive(Debug)]
struct Tone {
    tone: Option<String>,
    positive_score: Option<String>,
    negative_score: Option<String>,
    polarity: Option<String>,
    activity_reference_density: Option<String>,
    self_group_reference_density: Option<String>,
    word_count: Option<String>,
}

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

#[derive(Debug)]
struct Gcam {
    gcam_code: Option<String>,
    gcam_value: Option<f64>,
}

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
    fn test() {
        let file = File::open("resources/20230417163000.translation.gkg.csv").unwrap();
        let reader = BufReader::new(
            DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1252))
                .build(file));;

        for line in reader.lines() {
            let record_str = line.unwrap();
            let out: GKGEventV2 = parse_gkg_v2(&record_str);

            // process the record
            println!("{:?}", out.counts);
        }
    }

}



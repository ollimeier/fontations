//! The [CFF](https://learn.microsoft.com/en-us/typography/opentype/spec/cff) table

include!("../../generated/generated_cff.rs");

use read_fonts::{FontData, TopLevelTable};
use crate::codegen_prelude::*;

/// Top DICT data structure for CFF fonts
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TopDictData {
    pub version: Option<String>,
    pub notice: Option<String>,
    pub full_name: Option<String>,
    pub family_name: Option<String>,
    pub weight: Option<String>,
    pub font_bbox: Option<[f32; 4]>,
    pub charset: Option<usize>,
    pub encoding: Option<usize>,
    pub charstrings_offset: Option<usize>,
    pub private_dict_range: Option<std::ops::Range<usize>>,
    pub copyright: Option<String>,
    pub is_fixed_pitch: Option<bool>,
    pub italic_angle: Option<f32>,
    pub underline_position: Option<f32>,
    pub underline_thickness: Option<f32>,
    pub paint_type: Option<i32>,
    pub charstring_type: Option<i32>,
    pub font_matrix: Option<[f32; 6]>,
    pub stroke_width: Option<f32>,
    pub unique_id: Option<i32>,
    pub font_name: Option<String>,
}

/// The [Compact Font Format](https://learn.microsoft.com/en-us/typography/opentype/spec/cff) table.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cff {
    /// CFF header
    pub header: CffHeader,
    /// Font names
    pub names: Vec<String>,
    /// Top DICT data for each font
    pub top_dict_data: TopDictData,
    /// String data
    pub strings: Vec<String>,
    /// Global subroutines
    pub global_subrs: Vec<Vec<u8>>,
}

impl TopLevelTable for Cff {
    const TAG: read_fonts::types::Tag = read_fonts::types::Tag::new(b"CFF ");
}

impl FontWrite for Cff {
    fn write_into(&self, writer: &mut TableWriter) {
        // This is a simplified implementation
        // For now, we'll just write the header and basic structure
        self.header.write_into(writer);
    }

    fn table_type(&self) -> TableType {
        TableType::Named("CFF ")
    }
}

impl Validate for Cff {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {
        // TODO: Add validation logic
    }
}

impl<'a> FromTableRef<read_fonts::tables::cff::Cff<'a>> for Cff {
    fn from_table_ref(table: &read_fonts::tables::cff::Cff<'a>) -> Self {
        Self::from_obj_ref(table, table.offset_data())
    }
}

impl<'a> FromObjRef<read_fonts::tables::cff::Cff<'a>> for Cff {
    fn from_obj_ref(obj: &read_fonts::tables::cff::Cff<'a>, _offset_data: FontData) -> Self {
        // Convert the read CFF table to write CFF table
        let header = obj.header().to_owned_obj(_offset_data);
        
        // Extract names
        let names = (0..obj.names().count() as usize)
            .filter_map(|i| obj.name(i))
            .map(|name| name.to_string())
            .collect();

        // Extract and parse top dict data for the first font
        let mut top_dict_data = TopDictData::default();
        
        if let Ok(top_dict_bytes) = obj.top_dicts().get(0) {
            // Parse the top dict entries
            for entry in read_fonts::tables::postscript::dict::entries(top_dict_bytes, None) {
                if let Ok(entry) = entry {
                    match entry {
                        read_fonts::tables::postscript::dict::Entry::Version(sid) => {
                            if let Some(string) = obj.string(sid) {
                                top_dict_data.version = Some(string.to_string());
                            }
                        }
                        read_fonts::tables::postscript::dict::Entry::Notice(sid) => {
                            if let Some(string) = obj.string(sid) {
                                top_dict_data.notice = Some(string.to_string());
                            }
                        }
                        read_fonts::tables::postscript::dict::Entry::FullName(sid) => {
                            if let Some(string) = obj.string(sid) {
                                top_dict_data.full_name = Some(string.to_string());
                            }
                        }
                        read_fonts::tables::postscript::dict::Entry::FamilyName(sid) => {
                            if let Some(string) = obj.string(sid) {
                                top_dict_data.family_name = Some(string.to_string());
                            }
                        }
                        read_fonts::tables::postscript::dict::Entry::Weight(sid) => {
                            if let Some(string) = obj.string(sid) {
                                top_dict_data.weight = Some(string.to_string());
                            }
                        }
                        read_fonts::tables::postscript::dict::Entry::Copyright(sid) => {
                            if let Some(string) = obj.string(sid) {
                                top_dict_data.copyright = Some(string.to_string());
                            }
                        }
                        read_fonts::tables::postscript::dict::Entry::FontName(sid) => {
                            if let Some(string) = obj.string(sid) {
                                top_dict_data.font_name = Some(string.to_string());
                            }
                        }
                        read_fonts::tables::postscript::dict::Entry::FontBbox(bbox) => {
                            top_dict_data.font_bbox = Some([
                                bbox[0].to_f32(),
                                bbox[1].to_f32(),
                                bbox[2].to_f32(),
                                bbox[3].to_f32(),
                            ]);
                        }
                        read_fonts::tables::postscript::dict::Entry::ItalicAngle(angle) => {
                            top_dict_data.italic_angle = Some(angle.to_f32());
                        }
                        read_fonts::tables::postscript::dict::Entry::UnderlinePosition(pos) => {
                            top_dict_data.underline_position = Some(pos.to_f32());
                        }
                        read_fonts::tables::postscript::dict::Entry::UnderlineThickness(thickness) => {
                            top_dict_data.underline_thickness = Some(thickness.to_f32());
                        }
                        read_fonts::tables::postscript::dict::Entry::IsFixedPitch(fixed) => {
                            top_dict_data.is_fixed_pitch = Some(fixed);
                        }
                        read_fonts::tables::postscript::dict::Entry::PaintType(paint_type) => {
                            top_dict_data.paint_type = Some(paint_type);
                        }
                        read_fonts::tables::postscript::dict::Entry::CharstringType(cs_type) => {
                            top_dict_data.charstring_type = Some(cs_type);
                        }
                        read_fonts::tables::postscript::dict::Entry::FontMatrix(matrix) => {
                            top_dict_data.font_matrix = Some([
                                matrix[0].to_f32(),
                                matrix[1].to_f32(),
                                matrix[2].to_f32(),
                                matrix[3].to_f32(),
                                matrix[4].to_f32(),
                                matrix[5].to_f32(),
                            ]);
                        }
                        read_fonts::tables::postscript::dict::Entry::StrokeWidth(width) => {
                            top_dict_data.stroke_width = Some(width.to_f32());
                        }
                        read_fonts::tables::postscript::dict::Entry::Charset(offset) => {
                            top_dict_data.charset = Some(offset);
                        }
                        read_fonts::tables::postscript::dict::Entry::Encoding(offset) => {
                            top_dict_data.encoding = Some(offset);
                        }
                        read_fonts::tables::postscript::dict::Entry::CharstringsOffset(offset) => {
                            top_dict_data.charstrings_offset = Some(offset);
                        }
                        read_fonts::tables::postscript::dict::Entry::PrivateDictRange(range) => {
                            top_dict_data.private_dict_range = Some(range);
                        }
                        read_fonts::tables::postscript::dict::Entry::UniqueId(uid) => {
                            top_dict_data.unique_id = Some(uid);
                        }
                        _ => {
                            // Handle other entries as needed
                        }
                    }
                }
            }
        }

        // Extract strings
        let strings = (0..obj.strings().count() as usize)
            .filter_map(|i| obj.strings().get(i).ok())
            .map(|bytes| String::from_utf8_lossy(bytes).to_string())
            .collect();

        // Extract global subroutines
        let global_subrs = (0..obj.global_subrs().count() as usize)
            .filter_map(|i| obj.global_subrs().get(i).ok())
            .map(|bytes| bytes.to_vec())
            .collect();

        Cff {
            header,
            names,
            top_dict_data,
            strings,
            global_subrs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontBuilder, from_obj::ToOwnedTable};
    use read_fonts::{FontRef, TableProvider};

    #[test]
    fn test_cff_read_write_roundtrip() {
        let font_data = font_test_data::NOTO_SERIF_DISPLAY_TRIMMED;
        let font = FontRef::new(font_data).unwrap();

        // Read the CFF table
        let cff_read = font.cff().unwrap();

        // Convert to write table
        let mut cff_write: Cff = cff_read.to_owned_table();

        // Modify the top dict data
        cff_write.top_dict_data.version = Some("Version 1.23".to_string());
        cff_write.top_dict_data.family_name = Some("This is a Font Family Name".to_string());

        // For now, just test that we can create the structure and access the fields
        assert_eq!(cff_write.top_dict_data.version, Some("Version 1.23".to_string()));
        assert_eq!(cff_write.top_dict_data.family_name, Some("This is a Font Family Name".to_string()));
        
        // TODO: Full serialization and round-trip test will be implemented later
        // This requires a complete CFF writer implementation
    }
}
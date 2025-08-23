//! The [CFF2](https://learn.microsoft.com/en-us/typography/opentype/spec/cff2) table

include!("../../generated/generated_cff2.rs");

use read_fonts::{FontData, TopLevelTable};
use crate::codegen_prelude::*;

/// Top DICT data structure for CFF2 fonts  
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TopDictData {
    pub version: Option<String>,
    pub notice: Option<String>,
    pub full_name: Option<String>,
    pub family_name: Option<String>,
    pub weight: Option<String>,
    pub font_bbox: Option<[f32; 4]>,
    pub charstrings_offset: Option<usize>,
    pub variation_store_offset: Option<usize>,
    pub fd_array_offset: Option<usize>,
    pub fd_select_offset: Option<usize>,
    pub copyright: Option<String>,
    pub is_fixed_pitch: Option<bool>,
    pub italic_angle: Option<f32>,
    pub underline_position: Option<f32>,
    pub underline_thickness: Option<f32>,
    pub paint_type: Option<i32>,
    pub charstring_type: Option<i32>,
    pub font_matrix: Option<[f32; 6]>,
    pub stroke_width: Option<f32>,
    pub font_name: Option<String>,
}

/// The [Compact Font Format version 2](https://learn.microsoft.com/en-us/typography/opentype/spec/cff2) table.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cff2 {
    /// CFF2 header
    pub header: Cff2Header,
    /// Top DICT data
    pub top_dict_data: TopDictData,
    /// Global subroutines
    pub global_subrs: Vec<Vec<u8>>,
}

impl TopLevelTable for Cff2 {
    const TAG: read_fonts::types::Tag = read_fonts::types::Tag::new(b"CFF2");
}

impl FontWrite for Cff2 {
    fn write_into(&self, writer: &mut TableWriter) {
        // This is a simplified implementation
        // For now, we'll just write the header and basic structure
        self.header.write_into(writer);
    }

    fn table_type(&self) -> TableType {
        TableType::Named("CFF2")
    }
}

impl Validate for Cff2 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {
        // TODO: Add validation logic
    }
}

impl<'a> FromTableRef<read_fonts::tables::cff2::Cff2<'a>> for Cff2 {
    fn from_table_ref(table: &read_fonts::tables::cff2::Cff2<'a>) -> Self {
        Self::from_obj_ref(table, FontData::new(&[]))
    }
}

impl<'a> FromObjRef<read_fonts::tables::cff2::Cff2<'a>> for Cff2 {
    fn from_obj_ref(obj: &read_fonts::tables::cff2::Cff2<'a>, _offset_data: FontData) -> Self {
        // Convert the read CFF2 table to write CFF2 table
        let header = obj.header().to_owned_obj(_offset_data);
        
        // Parse the top dict data
        let mut top_dict_data = TopDictData::default();
        
        // CFF2 stores top dict data differently - it's in the header
        let top_dict_bytes = obj.top_dict_data();
        
        // Parse the top dict entries - CFF2 doesn't have strings index
        for entry in read_fonts::tables::postscript::dict::entries(top_dict_bytes, None) {
            if let Ok(entry) = entry {
                match entry {
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
                    read_fonts::tables::postscript::dict::Entry::CharstringsOffset(offset) => {
                        top_dict_data.charstrings_offset = Some(offset);
                    }
                    read_fonts::tables::postscript::dict::Entry::VariationStoreOffset(offset) => {
                        top_dict_data.variation_store_offset = Some(offset);
                    }
                    read_fonts::tables::postscript::dict::Entry::FdArrayOffset(offset) => {
                        top_dict_data.fd_array_offset = Some(offset);
                    }
                    read_fonts::tables::postscript::dict::Entry::FdSelectOffset(offset) => {
                        top_dict_data.fd_select_offset = Some(offset);
                    }
                    _ => {
                        // Handle other entries as needed
                    }
                }
            }
        }

        // Extract global subroutines
        let global_subrs = (0..obj.global_subrs().count() as usize)
            .filter_map(|i| obj.global_subrs().get(i).ok())
            .map(|bytes| bytes.to_vec())
            .collect();

        Cff2 {
            header,
            top_dict_data,
            global_subrs,
        }
    }
}
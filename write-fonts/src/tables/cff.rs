//! The [CFF](https://learn.microsoft.com/en-us/typography/opentype/spec/cff) table

include!("../../generated/generated_cff.rs");

// Include generated postscript types
include!("../../generated/generated_postscript.rs");

/// The [Compact Font Format](https://learn.microsoft.com/en-us/typography/opentype/spec/cff) table.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cff {
    /// The CFF header.
    pub header: CffHeader,
    /// Name index containing PostScript names of all fonts.
    pub names: Index1,
    /// Top DICT index containing top-level DICTs for all fonts.
    pub top_dicts: Index1,
    /// String index containing all strings used by fonts.
    pub strings: Index1,
    /// Global subroutine index containing sub-programs.
    pub global_subrs: Index1,
}

impl Cff {
    /// Construct a new `Cff` table.
    pub fn new(
        header: CffHeader,
        names: Index1,
        top_dicts: Index1,
        strings: Index1,
        global_subrs: Index1,
    ) -> Self {
        Self {
            header,
            names,
            top_dicts,
            strings,
            global_subrs,
        }
    }
}

impl FontWrite for Cff {
    fn write_into(&self, writer: &mut TableWriter) {
        self.header.write_into(writer);
        self.names.write_into(writer);
        self.top_dicts.write_into(writer);
        self.strings.write_into(writer);
        self.global_subrs.write_into(writer);
    }

    fn table_type(&self) -> crate::table_type::TableType {
        crate::table_type::TableType::TopLevel(read_fonts::tables::cff::Cff::TAG)
    }
}

impl Validate for Cff {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        self.header.validate_impl(ctx);
        self.names.validate_impl(ctx);
        self.top_dicts.validate_impl(ctx);
        self.strings.validate_impl(ctx);
        self.global_subrs.validate_impl(ctx);
    }
}

impl<'a> FromObjRef<read_fonts::tables::cff::Cff<'a>> for Cff {
    fn from_obj_ref(obj: &read_fonts::tables::cff::Cff<'a>, data: FontData) -> Self {
        Cff {
            header: obj.header().to_owned_obj(data),
            names: obj.names().to_owned_obj(data),
            top_dicts: obj.top_dicts().to_owned_obj(data),
            strings: obj.strings().to_owned_obj(data),
            global_subrs: obj.global_subrs().to_owned_obj(data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::cff::Cff<'a>> for Cff {}

impl TopLevelTable for Cff {
    const TAG: Tag = read_fonts::tables::cff::Cff::TAG;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::from_obj::ToOwnedTable;
    use font_types::Tag;
    use read_fonts::{FontData, FontRef, TableProvider};

    #[test]
    fn read_write_cff_table() {
        // Use test data that contains a CFF table
        let font_data = font_test_data::NOTO_SERIF_DISPLAY_TRIMMED;
        let font = FontRef::new(font_data).unwrap();
        
        // Read the CFF table
        let cff_read = font.cff().unwrap();
        
        // Convert to write table
        let cff_write: Cff = cff_read.to_owned_table();
        
        // Serialize the table
        let serialized = crate::dump_table(&cff_write).unwrap();
        
        // Parse it back
        let reparsed = read_fonts::tables::cff::Cff::read(FontData::new(&serialized)).unwrap();
        
        // Basic validation that key properties match
        assert_eq!(cff_read.header().major(), reparsed.header().major());
        assert_eq!(cff_read.header().minor(), reparsed.header().minor());
        assert_eq!(cff_read.names().count(), reparsed.names().count());
        assert_eq!(cff_read.top_dicts().count(), reparsed.top_dicts().count());
        assert_eq!(cff_read.strings().count(), reparsed.strings().count());
        assert_eq!(cff_read.global_subrs().count(), reparsed.global_subrs().count());
    }

    #[test]
    fn read_write_cff_table_with_top_dict_entries() {
        use read_fonts::tables::postscript::dict::{self, Entry};
        
        // Use test data that contains a CFF table with specific Top DICT entries
        let font_data = font_test_data::NOTO_SERIF_DISPLAY_TRIMMED;
        let font = FontRef::new(font_data).unwrap();
        
        // Read the CFF table
        let cff_read = font.cff().unwrap();
        
        // Extract and verify original Top DICT entries
        let original_top_dict_data = cff_read.top_dicts().get(0).unwrap();
        let original_entries: Vec<_> = dict::entries(original_top_dict_data, None)
            .map(|entry| entry.unwrap())
            .collect();
        
        // Verify that we have some key expected entries in the original
        let has_version = original_entries.iter().any(|e| matches!(e, Entry::Version(_)));
        let has_notice = original_entries.iter().any(|e| matches!(e, Entry::Notice(_)));
        let has_copyright = original_entries.iter().any(|e| matches!(e, Entry::Copyright(_)));
        let has_full_name = original_entries.iter().any(|e| matches!(e, Entry::FullName(_)));
        let has_family_name = original_entries.iter().any(|e| matches!(e, Entry::FamilyName(_)));
        let has_font_bbox = original_entries.iter().any(|e| matches!(e, Entry::FontBbox(_)));
        let has_private_dict_range = original_entries.iter().any(|e| matches!(e, Entry::PrivateDictRange(_)));
        let has_charstrings_offset = original_entries.iter().any(|e| matches!(e, Entry::CharstringsOffset(_)));
        
        // This font should have these specific entries
        assert!(has_version, "Original font should have Version entry");
        assert!(has_notice, "Original font should have Notice entry");
        assert!(has_copyright, "Original font should have Copyright entry");
        assert!(has_full_name, "Original font should have FullName entry");
        assert!(has_family_name, "Original font should have FamilyName entry");
        assert!(has_font_bbox, "Original font should have FontBbox entry");
        assert!(has_private_dict_range, "Original font should have PrivateDictRange entry");
        assert!(has_charstrings_offset, "Original font should have CharstringsOffset entry");
        
        // Convert to write table
        let cff_write: Cff = cff_read.to_owned_table();
        
        // Serialize the table
        let serialized = crate::dump_table(&cff_write).unwrap();
        
        // Parse it back
        let reparsed = read_fonts::tables::cff::Cff::read(FontData::new(&serialized)).unwrap();
        
        // Extract Top DICT entries from reparsed table
        let reparsed_top_dict_data = reparsed.top_dicts().get(0).unwrap();
        let reparsed_entries: Vec<_> = dict::entries(reparsed_top_dict_data, None)
            .map(|entry| entry.unwrap())
            .collect();
        
        // Verify that all entries from the original are preserved after round-trip
        for original_entry in &original_entries {
            let matching_reparsed = reparsed_entries.iter().find(|&e| {
                std::mem::discriminant(e) == std::mem::discriminant(original_entry)
            });
            
            // Every original entry should have a matching entry in the reparsed table
            assert!(
                matching_reparsed.is_some(),
                "Entry {:?} should be preserved after round-trip",
                original_entry
            );
            
            // Check specific values match for key entry types
            match (original_entry, matching_reparsed.unwrap()) {
                (Entry::Version(orig_id), Entry::Version(reparsed_id)) => {
                    assert_eq!(orig_id, reparsed_id, "Version StringId should match");
                }
                (Entry::Notice(orig_id), Entry::Notice(reparsed_id)) => {
                    assert_eq!(orig_id, reparsed_id, "Notice StringId should match");
                }
                (Entry::Copyright(orig_id), Entry::Copyright(reparsed_id)) => {
                    assert_eq!(orig_id, reparsed_id, "Copyright StringId should match");
                }
                (Entry::FullName(orig_id), Entry::FullName(reparsed_id)) => {
                    assert_eq!(orig_id, reparsed_id, "FullName StringId should match");
                }
                (Entry::FamilyName(orig_id), Entry::FamilyName(reparsed_id)) => {
                    assert_eq!(orig_id, reparsed_id, "FamilyName StringId should match");
                }
                (Entry::FontBbox(orig_bbox), Entry::FontBbox(reparsed_bbox)) => {
                    assert_eq!(orig_bbox, reparsed_bbox, "FontBbox should match");
                }
                (Entry::PrivateDictRange(orig_range), Entry::PrivateDictRange(reparsed_range)) => {
                    assert_eq!(orig_range, reparsed_range, "PrivateDictRange should match");
                }
                (Entry::CharstringsOffset(orig_offset), Entry::CharstringsOffset(reparsed_offset)) => {
                    assert_eq!(orig_offset, reparsed_offset, "CharstringsOffset should match");
                }
                (Entry::ItalicAngle(orig_angle), Entry::ItalicAngle(reparsed_angle)) => {
                    assert_eq!(orig_angle, reparsed_angle, "ItalicAngle should match");
                }
                (Entry::UnderlinePosition(orig_pos), Entry::UnderlinePosition(reparsed_pos)) => {
                    assert_eq!(orig_pos, reparsed_pos, "UnderlinePosition should match");
                }
                (Entry::UnderlineThickness(orig_thick), Entry::UnderlineThickness(reparsed_thick)) => {
                    assert_eq!(orig_thick, reparsed_thick, "UnderlineThickness should match");
                }
                (Entry::Weight(orig_weight), Entry::Weight(reparsed_weight)) => {
                    assert_eq!(orig_weight, reparsed_weight, "Weight StringId should match");
                }
                (Entry::Charset(orig_charset), Entry::Charset(reparsed_charset)) => {
                    assert_eq!(orig_charset, reparsed_charset, "Charset should match");
                }
                (Entry::FontMatrix(orig_matrix), Entry::FontMatrix(reparsed_matrix)) => {
                    assert_eq!(orig_matrix, reparsed_matrix, "FontMatrix should match");
                }
                _ => {} // Other entries - just having them preserved is sufficient
            }
        }
        
        // The number of entries should also match
        assert_eq!(
            original_entries.len(),
            reparsed_entries.len(),
            "Number of Top DICT entries should match after round-trip"
        );
    }

    #[test]
    fn write_cff_table_with_additional_top_dict_entries() {
        use read_fonts::tables::postscript::dict::{self, Entry};
        
        // Use test data that contains a CFF table
        let font_data = font_test_data::NOTO_SERIF_DISPLAY_TRIMMED;
        let font = FontRef::new(font_data).unwrap();
        
        // Read the CFF table
        let cff_read = font.cff().unwrap();
        
        // Convert to write table
        let cff_write: Cff = cff_read.to_owned_table();
        
        // For this test, we'll verify that the current implementation properly handles
        // various Top DICT entry types when they exist. Since our test font doesn't
        // have ItalicAngle or UnderlinePosition, we'll test what we can and validate
        // that the structure correctly preserves all entry types.
        
        // Serialize the table
        let serialized = crate::dump_table(&cff_write).unwrap();
        
        // Parse it back
        let reparsed = read_fonts::tables::cff::Cff::read(FontData::new(&serialized)).unwrap();
        
        // Test that we can handle reading various Top DICT entry types
        let original_top_dict_data = cff_read.top_dicts().get(0).unwrap();
        let original_entries: Vec<_> = dict::entries(original_top_dict_data, None)
            .map(|entry| entry.unwrap())
            .collect();
        
        let reparsed_top_dict_data = reparsed.top_dicts().get(0).unwrap();
        let reparsed_entries: Vec<_> = dict::entries(reparsed_top_dict_data, None)
            .map(|entry| entry.unwrap())
            .collect();
        
        // Verify comprehensive coverage of different entry types that may exist
        for entry in &original_entries {
            match entry {
                Entry::Version(_) => { /* Already tested in main test */ },
                Entry::Notice(_) => { /* Already tested in main test */ },
                Entry::FullName(_) => { /* Already tested in main test */ },
                Entry::FamilyName(_) => { /* Already tested in main test */ },
                Entry::Weight(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::Weight(_))));
                },
                Entry::FontBbox(_) => { /* Already tested in main test */ },
                Entry::CharstringsOffset(_) => { /* Already tested in main test */ },
                Entry::PrivateDictRange(_) => { /* Already tested in main test */ },
                Entry::VariationStoreOffset(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::VariationStoreOffset(_))));
                },
                Entry::Copyright(_) => { /* Already tested in main test */ },
                Entry::IsFixedPitch(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::IsFixedPitch(_))));
                },
                Entry::ItalicAngle(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::ItalicAngle(_))));
                },
                Entry::UnderlinePosition(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::UnderlinePosition(_))));
                },
                Entry::UnderlineThickness(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::UnderlineThickness(_))));
                },
                Entry::PaintType(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::PaintType(_))));
                },
                Entry::CharstringType(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::CharstringType(_))));
                },
                Entry::FontMatrix(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::FontMatrix(_))));
                },
                Entry::StrokeWidth(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::StrokeWidth(_))));
                },
                Entry::FdArrayOffset(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::FdArrayOffset(_))));
                },
                Entry::FdSelectOffset(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::FdSelectOffset(_))));
                },
                Entry::Encoding(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::Encoding(_))));
                },
                Entry::Charset(_) => { /* Already tested in main test */ },
                Entry::UniqueId(_) => {
                    // If present, verify it's preserved
                    assert!(reparsed_entries.iter().any(|e| matches!(e, Entry::UniqueId(_))));
                },
                _ => {
                    // For any other entry types, verify they're preserved
                    assert!(
                        reparsed_entries.iter().any(|e| std::mem::discriminant(e) == std::mem::discriminant(entry)),
                        "Entry type {:?} should be preserved after round-trip",
                        entry
                    );
                }
            }
        }
        
        // Verify comprehensive entry count
        assert_eq!(
            original_entries.len(),
            reparsed_entries.len(),
            "All Top DICT entries should be preserved"
        );
    }

    #[test]
    fn cff_table_tag() {
        assert_eq!(Cff::TAG, Tag::new(b"CFF "));
    }
}
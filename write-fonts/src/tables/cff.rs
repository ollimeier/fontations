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
    fn cff_table_tag() {
        assert_eq!(Cff::TAG, Tag::new(b"CFF "));
    }
}
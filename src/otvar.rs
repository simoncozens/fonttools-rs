use otspec::de::CountedDeserializer;
use otspec::ser;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::SerializeSeq;
use serde::Deserializer;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
extern crate otspec;
use otspec::types::*;
use otspec::{deserialize_visitor, read_field, read_field_counted, read_remainder};
use otspec_macros::tables;

tables!(
    RegionAxisCoordinates {
        F2DOT14	startCoord
        F2DOT14	peakCoord
        F2DOT14	endCoord
    }
    ItemVariationDataHeader {
        uint16	itemCount
        uint16	shortDeltaCount
        Counted(uint16) regionIndexes
    }

);

#[derive(Debug, PartialEq)]
pub struct ItemVariationData {
    regionIndexes: Vec<uint16>,
    deltaValues: Vec<Vec<int16>>,
}

deserialize_visitor!(
    ItemVariationData,
    ItemVariationDataVisitor,
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let header = read_field!(seq, ItemVariationDataHeader, "a header");
        let regionIndexCount = header.regionIndexes.len();
        let mut deltaValues = vec![];
        for _ in 0..header.itemCount {
            let mut v: Vec<i16> = Vec::new();
            for col in 0..regionIndexCount {
                if col <= header.shortDeltaCount as usize {
                    v.push(read_field!(seq, i16, "a delta"));
                } else {
                    v.push(read_field!(seq, i8, "a delta").into());
                }
            }
            deltaValues.push(v);
        }
        Ok(ItemVariationData {
            regionIndexes: header.regionIndexes,
            deltaValues,
        })
    }
);

struct VariationRegionList {
    axisCount: uint16,
    regionCount: uint16,
    variationRegions: Vec<Vec<RegionAxisCoordinates>>,
}
deserialize_visitor!(
    VariationRegionList,
    VariationRegionListVisitor,
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let axisCount = read_field!(seq, uint16, "an axis count");
        let regionCount = read_field!(seq, uint16, "a region count");
        let mut variationRegions = Vec::with_capacity(regionCount.into());
        for _ in 0..regionCount {
            let v: Vec<RegionAxisCoordinates> =
                read_field_counted!(seq, axisCount, "a VariationRegion record");
            variationRegions.push(v)
        }
        Ok(VariationRegionList {
            axisCount,
            regionCount,
            variationRegions,
        })
    }
);

#[derive(Debug, PartialEq)]
pub struct ItemVariationStore {
    format: uint16,
    axisCount: uint16,
    variationRegions: Vec<Vec<RegionAxisCoordinates>>,
    variationData: Vec<ItemVariationData>,
}

deserialize_visitor!(
    ItemVariationStore,
    ItemVariationStoreVisitor,
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let format = read_field!(seq, uint16, "a header");
        let offset = read_field!(seq, uint32, "an offset");
        let vardatacount = read_field!(seq, uint16, "a count") as usize;
        let variationDataOffsets: Vec<uint32> =
            read_field_counted!(seq, vardatacount, "item variation data offsets");
        let remainder = read_remainder!(seq, "an item variation store");
        let binary_variation_region_list =
            &remainder[offset as usize - (8 + 4 * vardatacount as usize)..];
        let variationRegions: VariationRegionList =
            otspec::de::from_bytes(binary_variation_region_list).map_err(|e| {
                serde::de::Error::custom(format!("Expecting a variation region list: {:?}", e))
            })?;
        let mut variationData = Vec::with_capacity(vardatacount);
        for i in 0..vardatacount {
            let vardata_binary =
                &remainder[variationDataOffsets[i] as usize - (8 + 4 * vardatacount as usize)..];
            variationData.push(otspec::de::from_bytes(vardata_binary).map_err(|e| {
                serde::de::Error::custom(format!("Expecting variation data: {:?}", e))
            })?);
        }
        Ok(ItemVariationStore {
            format,
            axisCount: variationRegions.axisCount,
            variationRegions: variationRegions.variationRegions,
            variationData,
        })
    }
);

#[cfg(test)]
mod tests {
    use crate::otvar;

    #[test]
    fn otvar_de_ivd() {
        let binary_ivd = vec![
            0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0x38, 0xFF, 0xCE, 0x00, 0x64,
            0x00, 0xC8,
        ];
        let fivd = otvar::ItemVariationData {
            regionIndexes: vec![0],
            deltaValues: vec![vec![-200], vec![-50], vec![100], vec![200]],
        };
        let deserialized: otvar::ItemVariationData = otspec::de::from_bytes(&binary_ivd).unwrap();
        assert_eq!(deserialized, fivd);
    }

    #[test]
    fn otvar_de_ivs() {
        let binary_ivs = vec![
            0x00, 0x01, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x01, 0x00, 0x00, 0x00, 0x16, 0x00, 0x01,
            0x00, 0x01, 0x00, 0x00, 0x40, 0x00, 0x40, 0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01,
            0x00, 0x00, 0xFF, 0x38, 0xFF, 0xCE, 0x00, 0x64, 0x00, 0xC8,
        ];
        let deserialized: otvar::ItemVariationStore = otspec::de::from_bytes(&binary_ivs).unwrap();
        let fivd = otvar::ItemVariationData {
            regionIndexes: vec![0],
            deltaValues: vec![vec![-200], vec![-50], vec![100], vec![200]],
        };
        let fivs = otvar::ItemVariationStore {
            format: 1,
            axisCount: 1,
            variationRegions: vec![vec![otvar::RegionAxisCoordinates {
                startCoord: 0.0,
                peakCoord: 1.0,
                endCoord: 1.0,
            }]],
            variationData: vec![fivd],
        };
        assert_eq!(deserialized, fivs);
    }
}
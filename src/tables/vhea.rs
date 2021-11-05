use otspec::types::*;
use otspec::Deserializer;
use otspec_macros::tables;

/// The 'vhea' OpenType tag.
pub const TAG: Tag = crate::tag!("vhea");

tables!(vhea {
    uint16 majorVersion
    uint16 minorVersion
    int16  vertTypoAscender
    int16  vertTypoDescender
    int16  vertTypoLineGap
    int16  advanceHeightMax
    int16   minTopSideBearing
    int16   minBottomSideBearing
    int16   yMaxExtent
    int16   caretSlopeRise
    int16   caretSlopeRun
    int16   caretOffset
    int16   reserved0
    int16   reserved1
    int16   reserved2
    int16   reserved3
    int16   metricDataFormat
    uint16  numberOfHMetrics
});

#[cfg(test)]
mod tests {
    use otspec::ser;

    #[test]
    fn vhea_ser() {
        let fvhea = super::vhea {
            majorVersion: 1,
            minorVersion: 0,
            vertTypoAscender: 705,
            vertTypoDescender: -180,
            vertTypoLineGap: 0,
            advanceHeightMax: 1311,
            minTopSideBearing: -382,
            minBottomSideBearing: -382,
            yMaxExtent: 1245,
            caretSlopeRise: 1,
            caretSlopeRun: 0,
            caretOffset: 0,
            reserved0: 0,
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            metricDataFormat: 0,
            numberOfHMetrics: 1117,
        };
        let binary_vhea = vec![
            0x00, 0x01, 0x00, 0x00, 0x02, 0xc1, 0xff, 0x4c, 0x00, 0x00, 0x05, 0x1f, 0xfe, 0x82,
            0xfe, 0x82, 0x04, 0xdd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x5d,
        ];
        assert_eq!(ser::to_bytes(&fvhea).unwrap(), binary_vhea);
    }

    #[test]
    fn vhea_de() {
        let fhhea = super::vhea {
            majorVersion: 1,
            minorVersion: 0,
            vertTypoAscender: 705,
            vertTypoDescender: -180,
            vertTypoLineGap: 0,
            advanceHeightMax: 1311,
            minTopSideBearing: -382,
            minBottomSideBearing: -382,
            yMaxExtent: 1245,
            caretSlopeRise: 1,
            caretSlopeRun: 0,
            caretOffset: 0,
            reserved0: 0,
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            metricDataFormat: 0,
            numberOfHMetrics: 1117,
        };
        let binary_vhea = vec![
            0x00, 0x01, 0x00, 0x00, 0x02, 0xc1, 0xff, 0x4c, 0x00, 0x00, 0x05, 0x1f, 0xfe, 0x82,
            0xfe, 0x82, 0x04, 0xdd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x5d,
        ];
        let deserialized: super::vhea = otspec::de::from_bytes(&binary_vhea).unwrap();
        assert_eq!(deserialized, fhhea);
    }
}

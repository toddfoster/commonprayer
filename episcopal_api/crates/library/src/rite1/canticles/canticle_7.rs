use canticle_table::CanticleId;
use liturgy::{Canticle, CanticleSection, CanticleVerse, Document, Version};

lazy_static! {
    pub static ref CANTICLE_7: Document = Document::from(Canticle {
        number: CanticleId::Canticle7,
        citation: None,
        local_name: String::from("We Praise Thee"),
        latin_name: Some(String::from("Te Deum laudamus")),
        rubric: None,
        sections: vec![CanticleSection {
            title: None,
            verses: vec![
                CanticleVerse::from((
                    "We praise thee, O God; we acknowledge thee to be the Lord.\nAll the earth doth worship thee, the Father everlasting.\nTo thee all Angels cry aloud,\nthe Heavens and all the Powers therein.\nTo thee Cherubim and Seraphim continually do cry:\n    Holy, holy, holy, Lord God of Sabaoth;\n    Heaven and earth are full of the majesty of thy glory.\nThe glorious company of the apostles praise thee.\nThe goodly fellowship of the prophets praise thee.\nThe noble army of martyrs praise thee.\nThe holy Church throughout all the world\n            doth acknowledge thee,\n    the Father, of an infinite majesty,\n    thine adorable, true, and only Son,\n    also the Holy Ghost the Comforter.\n\nThou art the King of glory, O Christ.\nThou art the everlasting Son of the Father.\nWhen thou tookest upon thee to deliver man,\nthou didst humble thyself to be born of a Virgin.\nWhen thou hadst overcome the sharpness of death,\nthou didst open the kingdom of heaven to all believers.\nThou sittest at the right hand of God, in the glory of the Father.\nWe believe that thou shalt come to be our judge.\n    We therefore pray thee, help thy servants,\n    whom thou hast redeemed with thy precious blood.\n    Make them to be numbered with thy saints,\n    in glory everlasting.",
                    ""
                ))
            ]
        }],
        gloria_patri: None,
    })
    .version(Version::RiteI)
    .page(52);
}

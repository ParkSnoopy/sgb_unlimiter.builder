use crate::localutils::{ decoder, printer::{ error, info } };


pub fn build() -> Vec<String> {
    info( "Building target vector from pre-prepared bytes...", None );

    let bytes = decoder::decode("EHQ&*E--2@:2OENF*)G@G@b5lB4Yt&:2OENF*)G@G@b6)G&g>qBP)-n@sWH<:2XZ^GA;AJAn1");

    if bytes.is_err() {
        error( "Decoder decode failed", None );
        panic!();
    }

    info( "Decode success!", None );

    String::from_utf8(bytes.unwrap()).unwrap().split("N").map(str::to_string).collect()
}

pub fn santinize(pname: &String) -> String {
    pname.split(".").collect::<Vec<&str>>()[0].to_lowercase()
}

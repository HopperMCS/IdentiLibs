fn main() {

    enum IsGender {
        Yes,
        No,
    }

    fn is_gender(is_gender: IsGender) -> String {
      match is_gender {
        IsGender::Yes => String::from("Status: Is A Gender"),
        IsGender::No => String::from("Status: NOT A Fucking Gender"),
      }
    }

    enum Gender {
        Agender,
        Androgyne,
        Androgynous,
        Bigender,
        Cis,
        Cisgender,
        CisFemale,
        CisMale,
        CisMan,
        CisWoman,
        CisgenderFemale,
        CisgenderMale,
        CisgenderMan,
        CisgenderWoman,
        FemaleToMale,
        FTM,
        GenderFluid,
        GenderNonconforming,
        GenderQuestioning,
        GenderVariant,
        Genderqueer,
        Intersex,
        MaleToFemale,
        MTF,
        Neither,
        Neutrois,
        NonBinary,
        Other,
        Pangender,
        Trans,
        TransAst,
        TransFemale,
        TransAstFemale,
        TransMale,
        TransAstMale,
        TransMan,
        TransAstMan,
        TransPerson,
        TransAstPerson,
        TransWoman,
        TransAstWoman,
        Transfeminine,
        Transgender,
        TransgenderFemale,
        TransgenderMale,
        TransgenderMan,
        TransgenderPerson,
        TransgenderWoman,
        Transmasculine,
        Transsexual,
        TranssexualFemale,
        TranssexualMale,
        TranssexualMan,
        TranssexualPerson,
        TranssexualWoman,
        TwoSpirit,
        Male,
        Female,
    }

    fn verify_gender(gender: Gender) -> String {

        match gender {
            Gender::Agender => String::from("Agender"),
            Gender::Androgyne => String::from("Androgyne"),
            Gender::Androgynous => String::from("Androgynous"),
            Gender::Bigender => String::from("Bi-gender"),
            Gender::Cis => String::from("Cis"),
            Gender::Cisgender => String::from("Cisgender"),
            Gender::CisFemale => String::from("Cis Female"),
            Gender::CisMale => String::from("Cis Male"),
            Gender::CisMan => String::from("Cis Man"),
            Gender::CisWoman => String::from("Cis Woman"),
            Gender::CisgenderFemale => String::from("Cisgender Female"),
            Gender::CisgenderMale => String::from("Cisgender Male"),
            Gender::CisgenderMan => String::from("Cisgender Man"),
            Gender::CisgenderWoman => String::from("Cisgender Woman"),
            Gender::FemaleToMale => String::from("Female To Male"),
            Gender::FTM => String::from("FTM"),
            Gender::GenderFluid => String::from("Gender Fluid"),
            Gender::GenderNonconforming => String::from("Gender Non-Conforming"),
            Gender::GenderQuestioning => String::from("Gender Questioning"),
            Gender::GenderVariant => String::from("Gender Variant"),
            Gender::Genderqueer => String::from("Genderqueer"),
            Gender::Intersex => String::from("Intersex"),
            Gender::MaleToFemale => String::from("Male-To-Female"),
            Gender::MTF => String::from("MTF"),
            Gender::Neither => String::from("Neither"),
            Gender::Neutrois => String::from("Neutrois"),
            Gender::NonBinary => String::from("Non-Binary"),
            Gender::Other => String::from("Other"),
            Gender::Pangender => String::from("Pangender"),
            Gender::Trans => String::from("Trans"),
            Gender::TransAst => String::from("Trans*"),
            Gender::TransFemale => String::from("Trans Female"),
            Gender::TransAstFemale => String::from("Trans* Female"),
            Gender::TransMale => String::from("Trans Male"),
            Gender::TransAstMale => String::from("Trans* Male"),
            Gender::TransMan => String::from("Trans Man"),
            Gender::TransAstMan => String::from("Trans* Man"),
            Gender::TransPerson => String::from("Trans Person"),
            Gender::TransAstPerson => String::from("Trans* Person"),
            Gender::TransWoman => String::from("Trans Woman"),
            Gender::TransAstWoman => String::from("Trans* Woman"),
            Gender::Transfeminine => String::from("Transfeminine"),
            Gender::Transgender => String::from("Transgender"),
            Gender::TransgenderFemale => String::from("Transgender Female"),
            Gender::TransgenderMale => String::from("Transgender Male"),
            Gender::TransgenderMan => String::from("Transgender Man"),
            Gender::TransgenderPerson => String::from("Transgender Person"),
            Gender::TransgenderWoman => String::from("Transgender Woman"),
            Gender::Transmasculine => String::from("Transmasculine"),
            Gender::Transsexual => String::from("Transsexual"),
            Gender::TranssexualFemale => String::from("Transsexual Female"),
            Gender::TranssexualMale => String::from("Transsexual Male"),
            Gender::TranssexualMan => String::from("Transsexual Man"),
            Gender::TranssexualPerson => String::from("Transsexual Person"),
            Gender::TranssexualWoman => String::from("Transsexual Woman"),
            Gender::TwoSpirit => String::from("Two-Spirit"),
            Gender::Male => String::from("Male"),
            Gender::Female => String::from("Female"),
        }
    }

    let yes = is_gender(IsGender::Yes);
    let no = is_gender(IsGender::No);

    let agender = verify_gender(Gender::Agender);
    println!("Gender: {}\n{}\n", agender, no);

    let androgyne = verify_gender(Gender::Androgyne);
    println!("Gender: {}\n{}\n", androgyne, no);

    let androgynous = verify_gender(Gender::Androgynous);
    println!("Gender: {}\n{}\n", androgynous, no);

    let bigender = verify_gender(Gender::Bigender);
    println!("Gender: {}\n{}\n", bigender, no);

    let cis = verify_gender(Gender::Cis);
    println!("Gender: {}\n{}\n", cis, no);

    let cisgender = verify_gender(Gender::Cisgender);
    println!("Gender: {}\n{}\n", cisgender, no);

    let cis_female = verify_gender(Gender::CisFemale);
    println!("Gender: {}\n{}\n", cis_female, no);

    let cis_man = verify_gender(Gender::CisMan);
    println!("Gender: {}\n{}\n", cis_man, no);

    let cis_male = verify_gender(Gender::CisMale);
    println!("Gender: {}\n{}\n", cis_male, no);

    let cis_woman = verify_gender(Gender::CisWoman);
    println!("Gender: {}\n{}\n", cis_woman, no);

    let cisgender_female = verify_gender(Gender::CisgenderFemale);
    println!("Gender: {}\n{}\n", cisgender_female, no);

    let cisgender_male = verify_gender(Gender::CisgenderMale);
    println!("Gender: {}\n{}\n", cisgender_male, no);

    let cisgender_man = verify_gender(Gender::CisgenderMan);
    println!("Gender: {}\n{}\n", cisgender_man, no);

    let cisgender_woman = verify_gender(Gender::CisgenderWoman);
    println!("Gender: {}\n{}\n", cisgender_woman, no);

    let female_to_male = verify_gender(Gender::FemaleToMale);
    println!("Gender: {}\n{}\n", female_to_male, no);

    let ftm = verify_gender(Gender::FTM);
    println!("Gender: {}\n{}\n", ftm, no);

    let gender_fluid = verify_gender(Gender::GenderFluid);
    println!("Gender: {}\n{}\n", gender_fluid, no);

    let gender_nonconforming = verify_gender(Gender::GenderNonconforming);
    println!("Gender: {}\n{}\n", gender_nonconforming, no);

    let gender_questioning = verify_gender(Gender::GenderQuestioning);
    println!("Gender: {}\n{}\n", gender_questioning, no); //18

    let gender_variant = verify_gender(Gender::GenderVariant);
    println!("Gender: {}\n{}\n", gender_variant, no);

    let gender_queer = verify_gender(Gender::Genderqueer);
    println!("Gender: {}\n{}\n", gender_queer, no);

    let intersex = verify_gender(Gender::Intersex);
    println!("Gender: {}\n{}\n", intersex, no);

    let male_to_female = verify_gender(Gender::MaleToFemale);
    println!("Gender: {}\n{}\n", male_to_female, no);

    let mtf = verify_gender(Gender::MTF);
    println!("Gender: {}\n{}\n", mtf, no);

    let neither = verify_gender(Gender::Neither);
    println!("Gender: {}\n{}\n", neither, no);

    let neutrois = verify_gender(Gender::Neutrois);
    println!("Gender: {}\n{}\n", neutrois, no);

    let non_binary = verify_gender(Gender::NonBinary);
    println!("Gender: {}\n{}\n", non_binary, no); //26

    let other = verify_gender(Gender::Other);
    println!("Gender: {}\n{}\n", other, no);

    let pangender = verify_gender(Gender::Pangender);
    println!("Gender: {}\n{}\n", pangender, no);

    let trans = verify_gender(Gender::Trans);
    println!("Gender: {}\n{}\n", trans, no);

    let trans_ast = verify_gender(Gender::TransAst);
    println!("Gender: {}\n{}\n", trans_ast, no); //30

    let trans_female = verify_gender(Gender::TransFemale);
    println!("Gender: {}\n{}\n", trans_female, no);

    let trans_ast_female = verify_gender(Gender::TransAstFemale);
    println!("Gender: {}\n{}\n", trans_ast_female, no);

    let trans_male = verify_gender(Gender::TransMale);
    println!("Gender: {}\n{}\n", trans_male, no);

    let trans_ast_male = verify_gender(Gender::TransAstMale);
    println!("Gender: {}\n{}\n", trans_ast_male, no); //34

    let trans_man = verify_gender(Gender::TransMan);
    println!("Gender: {}\n{}\n", trans_man, no);

    let trans_ast_man = verify_gender(Gender::TransAstMan);
    println!("Gender: {}\n{}\n", trans_ast_man, no);

    let trans_person = verify_gender(Gender::TransPerson);
    println!("Gender: {}\n{}\n", trans_person, no);

    let trans_ast_person = verify_gender(Gender::TransAstPerson);
    println!("Gender: {}\n{}\n", trans_ast_person, no); //38

    let trans_woman = verify_gender(Gender::TransWoman);
    println!("Gender: {}\n{}\n", trans_woman, no);

    let trans_ast_woman = verify_gender(Gender::TransAstWoman);
    println!("Gender: {}\n{}\n", trans_ast_woman, no);

    let transfeminine = verify_gender(Gender::Transfeminine);
    println!("Gender: {}\n{}\n", transfeminine, no);

    let transgender = verify_gender(Gender::Transgender);
    println!("Gender: {}\n{}\n", transgender, no); //42

    let transgender_female = verify_gender(Gender::TransgenderFemale);
    println!("Gender: {}\n{}\n", transgender_female, no);

    let transgender_male = verify_gender(Gender::TransgenderMale);
    println!("Gender: {}\n{}\n", transgender_male, no);

    let transgender_man = verify_gender(Gender::TransgenderMan);
    println!("Gender: {}\n{}\n", transgender_man, no);

    let transgender_person = verify_gender(Gender::TransgenderPerson);
    println!("Gender: {}\n{}\n", transgender_person, no); //46

    let transgender_woman = verify_gender(Gender::TransgenderWoman);
    println!("Gender: {}\n{}\n", transgender_woman, no);

    let transmasculine = verify_gender(Gender::Transmasculine);
    println!("Gender: {}\n{}\n", transmasculine, no);

    let transsexual = verify_gender(Gender::Transsexual);
    println!("Gender: {}\n{}\n", transsexual, no);

    let transsexual_female = verify_gender(Gender::TranssexualFemale);
    println!("Gender: {}\n{}\n", transsexual_female, no); //50

    let transsexual_male = verify_gender(Gender::TranssexualMale);
    println!("Gender: {}\n{}\n", transsexual_male, no);

    let transsexual_man = verify_gender(Gender::TranssexualMan);
    println!("Gender: {}\n{}\n", transsexual_man, no);

    let transsexual_person = verify_gender(Gender::TranssexualPerson);
    println!("Gender: {}\n{}\n", transsexual_person, no);

    let transsexual_woman = verify_gender(Gender::TranssexualWoman);
    println!("Gender: {}\n{}\n", transsexual_woman, no); //54

    let two_spirit = verify_gender(Gender::TwoSpirit);
    println!("Gender: {}\n{}\n", two_spirit, no);

    let male = verify_gender(Gender::Male);
    println!("Gender: {}\n{}\n", male, yes);

    let female = verify_gender(Gender::Female);
    println!("Gender: {}\n{}\n", female, yes);

}

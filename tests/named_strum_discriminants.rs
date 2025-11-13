mod works {
    use strum::EnumDiscriminants;
    use subenum::subenum;

    // This works
    #[subenum(
        Opunia(derive(EnumDiscriminants), strum_discriminants(name(OpuniaSpecies))),
        Pachycereus(
            derive(EnumDiscriminants),
            strum_discriminants(name(PachycereusSpecies))
        )
    )]
    #[derive(Clone, Copy)]
    pub enum Cactus {
        #[subenum(Opunia)]
        Abjecta,

        #[subenum(Opunia)]
        Azurea,

        #[subenum(Opunia)]
        Caracassana,

        #[subenum(Pachycereus)]
        Grandis,

        #[subenum(Pachycereus)]
        Pringlei,
    }

    #[test]
    fn cactii() {
        let abjecta = Opunia::Azurea;
        let _species: OpuniaSpecies = abjecta.into();
        let _cactus: Cactus = abjecta.into();
    }
}

/// This won't compile :(
mod doesnt_work {
    use strum::EnumDiscriminants;
    use subenum::subenum;

    #[derive(EnumDiscriminants)]
    #[strum_discriminants(name(CactusSpecies))]
    // ^^^ Note: #[derive] above #[subenum] doesn't derive on the generated types, but it seems
    // like `#[strum_discriminants(name(CactusSpecies))]` is being passed through.
    #[subenum(
        Opunia(derive(EnumDiscriminants), strum_discriminants(name(OpuniaSpecies))),
        Pachycereus(
            derive(EnumDiscriminants),
            strum_discriminants(name(PachycereusSpecies))
        )
    )]
    #[derive(Clone, Copy)]
    pub enum Cactus {
        #[subenum(Opunia)]
        Abjecta,

        #[subenum(Opunia)]
        Azurea,

        #[subenum(Opunia)]
        Caracassana,

        #[subenum(Pachycereus)]
        Grandis,

        #[subenum(Pachycereus)]
        Pringlei,
    }
}

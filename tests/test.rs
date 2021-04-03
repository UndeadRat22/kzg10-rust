use mcl_rust::*;
use std::mem;

macro_rules! field_test {
    ($t:ty) => {{
        let mut x = <$t>::zero();
        assert!(x.is_valid());
        assert!(x.is_zero());
        assert!(!x.is_one());
        x.set_int(1);
        assert!(!x.is_zero());
        assert!(x.is_one());
        let mut y = <$t>::from_int(1);
        assert!(y.is_valid());
        assert_eq!(x, y);
        y.set_int(2);
        assert!(x != y);
        x.set_str("65535", 10);
        y.set_str("ffff", 16);
        assert!(x.is_valid());
        assert_eq!(x, y);
        x.set_int(123);
        assert!(x.is_odd());
        x.set_int(124);
        assert!(!x.is_odd());
        assert!(!x.is_negative());
        x.set_int(-124);
        assert!(x.is_negative());

        let mut z = unsafe { <$t>::uninit() };
        let mut w = unsafe { <$t>::uninit() };

        let a = 256;
        let b = 8;
        x.set_int(a);
        y.set_int(b);
        <$t>::add(&mut z, &x, &y);
        w.set_int(a + b);
        assert_eq!(z, w);
        assert_eq!(w, (&x + &y));
        z = x.clone();
        z += &y;
        assert_eq!(z, w);

        <$t>::sub(&mut z, &x, &y);
        w.set_int(a - b);
        assert_eq!(z, w);
        assert_eq!(w, (&x - &y));
        z = x.clone();
        z -= &y;
        assert_eq!(z, w);

        <$t>::mul(&mut z, &x, &y);
        w.set_int(a * b);
        assert_eq!(z, w);
        assert_eq!(w, (&x * &y));
        z = x.clone();
        z *= &y;
        assert_eq!(z, w);

        <$t>::div(&mut z, &x, &y);
        w.set_int(a / b);
        assert_eq!(z, w);
        assert_eq!(z, (&x / &y));
        z = x.clone();
        z /= &y;
        assert_eq!(z, w);

        assert!(x.set_little_endian_mod(&[1, 2, 3, 4, 5]));
        assert_eq!(x.get_str(16), "504030201");
        <$t>::sqr(&mut y, &x);
        <$t>::mul(&mut z, &x, &x);
        assert_eq!(y, z);

        assert!(<$t>::square_root(&mut w, &y));
        if w != x {
            <$t>::neg(&mut z, &w);
            assert_eq!(x, z);
        }
    }};
}

macro_rules! ec_test {
    ($t:ty, $f:ty, $P:expr) => {
        #[allow(non_snake_case)]
        assert!($P.is_valid());
        assert!(!$P.is_zero());
        let mut P1 = <$t>::zero();
        assert!(P1.is_zero());
        assert_ne!(P1, $P);
        <$t>::neg(&mut P1, &$P);
        let mut x: $f = unsafe { <$f>::uninit() };
        <$f>::neg(&mut x, &P1.y);
        assert_eq!(&x, &$P.y);

        <$t>::dbl(&mut P1, &$P);
        let mut P2: $t = unsafe { <$t>::uninit() };
        let mut P3: $t = unsafe { <$t>::uninit() };
        <$t>::add(&mut P2, &$P, &$P);
        assert_eq!(P2, P1);
        <$t>::add(&mut P3, &P2, &$P);
        assert_eq!(P3, (&P2 + &$P));
        assert_eq!(P2, (&P3 - &$P));
        let mut y: Fr = Fr::from_int(1);
        <$t>::mul(&mut P2, &$P, &y);
        assert_eq!(P2, $P);
        y.set_int(2);
        <$t>::mul(&mut P2, &$P, &y);
        assert_eq!(P2, P1);
        y.set_int(3);
        <$t>::mul(&mut P2, &$P, &y);
        assert_eq!(P2, P3);
        P2 = P1.clone();
        P2 += &$P;
        assert_eq!(P2, P3);

        P2 -= &$P;
        assert_eq!(P2, P1);
    };
}

macro_rules! serialize_test {
    ($t:ty, $x:expr) => {
        let buf = $x.serialize();
        let mut y: $t = unsafe { <$t>::uninit() };
        assert!(y.deserialize(&buf));
        assert_eq!($x, y);
    };
}

#[test]
#[allow(non_snake_case)]
fn mcl_test() {
    assert_eq!(mem::size_of::<Fr>(), 32);
    assert_eq!(mem::size_of::<Fp>(), 48);
    assert_eq!(mem::size_of::<Fp2>(), 48 * 2);
    assert_eq!(mem::size_of::<G1>(), 48 * 3);
    assert_eq!(mem::size_of::<G2>(), 48 * 2 * 3);
    assert_eq!(mem::size_of::<GT>(), 48 * 12);
    assert!(init(CurveType::BLS12_381));
    assert_eq!(get_fp_serialized_size(), 48);
    assert_eq!(get_g1_serialized_size(), 48);
    assert_eq!(get_g2_serialized_size(), 48 * 2);
    assert_eq!(get_gt_serialized_size(), 48 * 12);
    assert_eq!(get_fr_serialized_size(), 32);

    // Fp
    assert_eq!(get_field_order(), "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787");
    // Fr
    assert_eq!(
        get_curve_order(),
        "52435875175126190479447740508185965837690552500527637822603658699938581184513"
    );

    field_test! {Fr};
    field_test! {Fp};

    let P = G1::from_str("1 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569", 10).unwrap();
    let Q = G2::from_str("1 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758 1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582", 10).unwrap();

    ec_test! {G1, Fp, P};
    ec_test! {G2, Fp2, Q};

    let x = Fr::from_int(3);
    let y = Fp::from_int(-1);
    let mut e = unsafe { GT::uninit() };
    pairing(&mut e, &P, &Q);
    serialize_test! {Fr, x};
    serialize_test! {Fp, y};
    serialize_test! {G1, P};
    serialize_test! {G2, Q};
    serialize_test! {GT, e};
}


#[test]
fn polynomial_new_works_with_valid_params() {
    // Arrange
    assert!(init(CurveType::BLS12_381));
    let coefficients = vec![1, 2, 3, 4, 7, 7, 7, 7, 13, 13, 13, 13, 13, 13, 13, 13];
    
    // Act
    // Assert
    let _poly = Polynomial::new(&coefficients);
}

const GEN_G1_STR: &str = "1 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569";
const GEN_G2_STR: &str = "1 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758 1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582";
#[test]
fn curve_new_sets_g1_gen_to_correct_val() {
    // Arrange
    assert!(init(CurveType::BLS12_381));
    // Act
    let curve = Curve::new(&Fr::one(), 1);
    // Assert
    let result = curve.g1_gen.get_str(10);
    assert_eq!(GEN_G1_STR, result);
}

#[test]
fn curve_new_sets_g2_gen_to_correct_val() {
    // Arrange
    assert!(init(CurveType::BLS12_381));
    // Act
    let curve = Curve::new(&Fr::one(), 1);
    // Assert
    let result = curve.g2_gen.get_str(10);
    assert_eq!(GEN_G2_STR, result);
}

#[test]
fn curve_new_first_g1_point_is_generator() {
    // Arrange
    assert!(init(CurveType::BLS12_381));
    // Act
    let curve = Curve::new(&Fr::one(), 1);
    // Assert
    let result = curve.g1_points[0].get_str(10);
    assert_eq!(GEN_G1_STR, result);
}

#[test]
fn curve_new_first_g2_point_is_generator() {
    // Arrange
    assert!(init(CurveType::BLS12_381));
    // Act
    let curve = Curve::new(&Fr::one(), 1);
    // Assert
    let result = curve.g2_points[0].get_str(10);
    assert_eq!(GEN_G2_STR, result);
}

#[test]
fn curve_new_g1_points_should_have_exact_values_given_specific_params() {
    // Arrange
    assert!(init(CurveType::BLS12_381));
    let secret = Fr::from_str("1927409816240961209460912649124", 10);
    // Act
    let curve = Curve::new(&secret.unwrap(), 17);

    // Assert
    let strs: Vec<String> = curve.g1_points.iter().map(|x| x.get_str(10)).collect();

    let expected = [
        "1 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569",
        "1 2152869591689196284448695346189574884812111481958382828258011215851576980696076283357586937048281406709538669668556 1837985193201176164563363095685788695060722952730414729513827449189998486108746221085867464994192483925080285065018",
        "1 3262605383935052083682746418410148264888435523851787883646428634651057127775051437848295094756022375626344454529853 170376908770402617588871716106905630704800483257535202301637743269417617389615424187202633271003852068580182760145",
        "1 880220465676249036392707021793508724521687313339128521628378026681982232143465812471317982023333854734330480948673 2902223523920951554723894475268175106169924753723683323415827815697283974835754478310015241899366201222645054649608",
        "1 3244388122438103216409647202891415823727015100716970678191779835123826603745878948836800400002635659162267031975583 1585253991742417487161783435845975575940496801371133300360989731905095816244520655199543596433385055483512833757601",
        "1 1691444095738545636838570153348610026182961225323356293730618623455617099056679194783075235346140714244554956873033 961647227525848937006956574637866372979536342809091069926681629924395961591459720348224030656076442822973218085550",
        "1 3577901166225087165094130311167725910816358129621364033783794190115179171035326630844229271546155259801088983862479 3092642789043285621253509221940543294037789204421586765911962911994791146080221763753835525158184924558752020565291",
        "1 3013186384424179439775834119326464143135586883979982227880894049363998889225981543565484795480442461763905556101762 2478178060772079991661592547244530979985617852562178845899785854640900915922275633937010001028451411707598232298832",
        "1 2399231498671876712809138864159907535047647996068498551664715372729570379868491143868576593216337700083434635379685 482713507023481840088214347482905793358357894612071198190992625150626572379700355268448916577827133516290766497622",
        "1 3969977694904317593192991000252371163313871217135365989839005125424170023895776071038962785493744895844713942612974 4413699249467038796028960432613629427107969427001375982715766204951543003390119367884220008325083276852454909781",
        "1 194075085492710730179274360855839046245139911409378180773739622831795665144267111066969149760214641537576331892106 2829985443790966823527351173358032012294116269530327251028202388989851835102944896169897747047401344807088160112615",
        "1 3280685826964465549968732412388399530552593104925613549928682916052288843943420728109800551779100636680533358924443 298681365483796231406540809779630027096164486961762590101806824599619177657034359801891143498721811778563760637265",
        "1 3400935219571983529219239890354858045697412824966239298938573345769483174728220352364464899823354648929772679742552 191348415359823743448098111145821414668135484237623052156975028197191932451915485624239658088947171507703851149457",
        "1 3374538774097525861555904858666250834790705983053158676667960471408213151238217454016543185985604089823231022873495 3258515632833107359343766033193307526227113423713039039660926225561152436287447196511646604996224783556594615862297",
        "1 84257718831135536599708664101759228548259087509946180939819270114426033813393924094069423610123184508861693807222 1425072665498715675588703937845375337020452759807133251766395540291673980878175955156843196123520432067102459055363",
        "1 3252002729706195882623612396123269947851274285378019889513328537606104266375911074185178476483143881219657281907365 2744825182574836694639045426047274581117055175119425124924870369929899567807025339868314937727954579104158715615767",
        "1 3105530220482690203381088120196667002396187280376878591204985144810501655541159693720070013410365841734894244215545 2515128161348588248190794853040865399542397670879510720694265058064715843238549344693260209850633864871507776822678"
        ];
    
    assert_eq!(expected.len(), strs.len());
    let all_values_equal = expected.iter().zip(strs.iter()).all(|(a, b)| a == b);
    assert!(all_values_equal);
}

#[test]
fn polynomial_generate_proof_at_should_have_a_specific_value_given_exact_inputs() {
    // Arrange
    assert!(init(CurveType::BLS12_381));
    let coefficients = vec![1, 2, 3, 4, 7, 7, 7, 7, 13, 13, 13, 13, 13, 13, 13, 13];
    let poly = Polynomial::new(&coefficients);
    let secret = Fr::from_str("1927409816240961209460912649124", 10);
    let curve = Curve::new(&secret.unwrap(), poly.order());
    
    let point = Fr::from_int(17);
    // Act
    let proof = poly.gen_proof_at(&curve.g1_points, &point);
    // Assert
    let expected = "1 867803339007397142967426903694725732786398875082812714585913536387867789215930966591756718433944432919654354450045 1056604647851765547809696011101405958529416282518275445556537937608095695960215709670255078026676619389223749112525";
    let actual = proof.get_str(10);
    assert_eq!(expected, actual);
}

#[test]
fn polynomial_commit_should_have_specific_value_given_exact_inputs() {
    // Arrange
    assert!(init(CurveType::BLS12_381));
    let coefficients = vec![1, 2, 3, 4, 7, 7, 7, 7, 13, 13, 13, 13, 13, 13, 13, 13];
    let poly = Polynomial::new(&coefficients);
    let secret = Fr::from_str("1927409816240961209460912649124", 10);
    let curve = Curve::new(&secret.unwrap(), poly.order());
    // Act
    let commitment = poly.commit(&curve.g1_points);
    // Assert
    let expected = "1 2477800657478396280496910737712311876776119382023479023481824166251818861301026600704438635866253640104679377733301 2954229993619572531997767690230550003591055333364019243777247691138518393343620011708288769387987906907914339131963";
    let actual = commitment.get_str(10);
    assert_eq!(expected, actual);
}

#[test]
fn polynomial_eval_at_should_specific_value_given_exact_inputs() {
        // Arrange
        assert!(init(CurveType::BLS12_381));
        let coefficients = vec![1, 2, 3, 4, 7, 7, 7, 7, 13, 13, 13, 13, 13, 13, 13, 13];
        let poly = Polynomial::new(&coefficients);
        // Act
        let value = poly.eval_at(&Fr::from_int(17));
        // Assert
        let expected = "39537218396363405614";
        let actual = value.get_str(10);
        assert_eq!(expected, actual);
}

#[test]
fn curve_is_proof_valid_should_return_true_when_same_parameters_used_for_gen_are_passed() {
    // Arrange
    assert!(init(CurveType::BLS12_381));
    let coefficients = vec![1, 2, 3, 4, 7, 7, 7, 7, 13, 13, 13, 13, 13, 13, 13, 13];
    let poly = Polynomial::new(&coefficients);
    let secret = Fr::from_str("1927409816240961209460912649124", 10);
    let curve = Curve::new(&secret.unwrap(), poly.order());
    let x = Fr::from_int(17);
    let y = poly.eval_at(&x);
    let proof = poly.gen_proof_at(&curve.g1_points, &x);
    let commitment = poly.commit(&curve.g1_points);

    // Act
    let is_valid = curve.is_proof_valid(&commitment, &proof, &x, &y);

    // Assert
    assert!(is_valid);
}

#[test]
fn proof_loop_works_with_random_points() {
        // Arrange
        assert!(init(CurveType::BLS12_381));
        let coefficients = vec![1, 2, 3, 4, 7, 7, 7, 7, 13, 13, 13, 13, 13, 13, 13, 13];
        let poly = Polynomial::new(&coefficients);
        let secret = Fr::from_str("1927409816240961209460912649124", 10);
        let curve = Curve::new(&secret.unwrap(), poly.order());
        let mut x = Fr::default();
        x.set_by_csprng();
        let y = poly.eval_at(&x);
        let proof = poly.gen_proof_at(&curve.g1_points, &x);
        let commitment = poly.commit(&curve.g1_points);
    
        // Act
        let is_valid = curve.is_proof_valid(&commitment, &proof, &x, &y);
    
        // Assert
        assert!(is_valid);
}
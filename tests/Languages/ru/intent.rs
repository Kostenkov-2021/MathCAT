/// Тесты для правил, общих для различных стилей речи:
/// * явный intent, русская локализация и грамматика аргументов
use crate::common::*;

use anyhow::Result;

fn all_russian_speech_modes(expr: &str, expected: &str) -> Result<()> {
    for style in ["ClearSpeak", "SimpleSpeak"] {
        for verbosity in ["Terse", "Medium", "Verbose"] {
            test_prefs("ru", style, vec![("Verbosity", verbosity), ("IntentErrorRecovery", "Error")], expr, expected)?;
        }
    }
    Ok(())
}

#[test]
fn w3c_point_dimensions() -> Result<()> {
    // W3C permits one or more coordinates; retain the established two-coordinate reading.
    all_russian_speech_modes("<math><mrow intent='point(1)'/></math>", "точка 1")?;
    all_russian_speech_modes("<math><mrow intent='point(1,2)'/></math>", "точка 1 запятая 2")?;
    all_russian_speech_modes("<math><mrow intent='point(1,2,3)'/></math>", "точка 1 запятая 2 запятая 3")
}

#[test]
fn w3c_point_compound_coordinate() -> Result<()> {
    // A coordinate expression remains a single coordinate, with the separator after the whole expression.
    all_russian_speech_modes("<math><mrow intent='point($c,3)'><mrow arg='c'><mn>1</mn><mo>+</mo><mn>2</mn></mrow></mrow></math>", "точка 1 плюс 2 запятая 3")
}

#[test]
fn w3c_pochhammer_function() -> Result<()> {
    // W3C's default function form keeps n=5 and k=2 in the Russian 'из n по k' order.
    all_russian_speech_modes("<math><mrow intent='pochhammer(5,2)'/></math>", "число размещений из 5 по 2")?;
    all_russian_speech_modes("<math><mrow intent='pochhammer:function(5,2)'/></math>", "число размещений из 5 по 2")
}

#[test]
fn w3c_pochhammer_infix_compatibility() -> Result<()> {
    // Existing inferred intents use infix fixity; their wording and argument order must remain unchanged.
    all_russian_speech_modes("<math><mrow intent='pochhammer:infix(5,2)'/></math>", "число размещений из 5 по 2")
}

#[test]
fn function_set_image() -> Result<()> {
    // An explicit image intent names the image, with a genitive capital-letter operand.
    all_russian_speech_modes("<math><mrow intent='image($r)'><mi>Im</mi><mi arg='r'>R</mi></mrow></math>", "образ заглавной эр")
}

#[test]
fn function_set_domain() -> Result<()> {
    // The domain is the set of allowed inputs, not an ordinary function applied with "от".
    all_russian_speech_modes("<math><mrow intent='domain($f)'><mi>Dom</mi><mi arg='f'>F</mi></mrow></math>", "область определения заглавной эф")
}

#[test]
fn function_set_codomain() -> Result<()> {
    // The codomain is the target set; it must remain distinct from the image.
    all_russian_speech_modes("<math><mrow intent='codomain($f)'><mi>Cod</mi><mi arg='f'>F</mi></mrow></math>", "целевое множество заглавной эф")
}

#[test]
fn function_set_nested_genitive() -> Result<()> {
    // Nested concept names themselves take the genitive, without an extra expression cue.
    all_russian_speech_modes("<math><mrow intent='image(domain($f))'><mi>Im</mi><mi arg='f'>F</mi></mrow></math>", "образ области определения заглавной эф")?;
    all_russian_speech_modes("<math><mrow intent='domain(codomain(image($f)))'><mi>Dom</mi><mi arg='f'>F</mi></mrow></math>", "область определения целевого множества образа заглавной эф")
}

#[test]
fn function_set_compound_operand() -> Result<()> {
    // A compound operand is introduced with genitive "выражения"; its internal letters stay nominative.
    all_russian_speech_modes("<math><mrow intent='image($f)'><mi>Im</mi><mrow arg='f'><mi>F</mi><mo>+</mo><mi>G</mi></mrow></mrow></math>", "образ выражения заглавная эф плюс заглавная гэ")
}

#[test]
fn function_set_name_only() -> Result<()> {
    // A standalone operator name has no operand to inflect and needs no connector.
    all_russian_speech_modes("<math><mi intent='image'>Im</mi></math>", "образ")?;
    all_russian_speech_modes("<math><mi intent='domain'>Dom</mi></math>", "область определения")?;
    all_russian_speech_modes("<math><mi intent='codomain'>Cod</mi></math>", "целевое множество")
}

#[test]
fn complex_parts_with_style_wrappers() -> Result<()> {
    // TeX4ht splits upright function names into mi children of mstyle; neither name may disappear.
    let expr = "<math><mi>a</mi><mo>=</mo><mstyle mathvariant='normal'><mi>R</mi><mi>e</mi></mstyle><mspace width='0.28em'/><mi>z</mi><mo>,</mo><mi>b</mi><mo>=</mo><mstyle mathvariant='normal'><mi>I</mi><mi>m</mi></mstyle><mspace width='0.28em'/><mi>z</mi></math>";
    test("ru", "ClearSpeak", expr, "а равно действительная часть от зэт запятая, бэ равно мнимая часть от зэт")?;
    test("ru", "SimpleSpeak", expr, "а равно действительная часть от зэт запятая, бэ равно мнимая часть от зэт")
}

#[test]
fn complex_parts_without_style_wrappers() -> Result<()> {
    // Access8Math removes mstyle wrappers; split Re and Im still denote functions of z.
    let expr = "<math><mi>a</mi><mo>=</mo><mi>R</mi><mi>e</mi><mspace width='0.28em'/><mi>z</mi><mo>,</mo><mi>b</mi><mo>=</mo><mi>I</mi><mi>m</mi><mspace width='0.28em'/><mi>z</mi></math>";
    test("ru", "ClearSpeak", expr, "а равно действительная часть от зэт запятая, бэ равно мнимая часть от зэт")?;
    test("ru", "SimpleSpeak", expr, "а равно действительная часть от зэт запятая, бэ равно мнимая часть от зэт")
}

#[test]
fn complex_parts_lowercase() -> Result<()> {
    // Lowercase re/im must be recognized just like Re/Im, without an explicit apply operator.
    let expr = "<math><mi>re</mi><mi>z</mi><mo>,</mo><mi>im</mi><mi>z</mi></math>";
    test("ru", "ClearSpeak", expr, "действительная часть от зэт запятая, мнимая часть от зэт")?;
    test("ru", "SimpleSpeak", expr, "действительная часть от зэт запятая, мнимая часть от зэт")
}

#[test]
fn imaginary_part_name() -> Result<()> {
    // The imaginary part is a real component of a complex number, not a "complex part".
    let expr = r#"<math><mi intent="imaginary-part">Im</mi></math>"#;
    test("ru", "SimpleSpeak", expr, "мнимая часть")?;
    test("ru", "ClearSpeak", expr, "мнимая часть")?;
    return Ok(());
}

#[test]
fn silent_intent() -> Result<()> {
    let expr = "<math> <mrow intent='testing:silent($arg1, $arg2)'><mn arg='arg1'>2</mn> <mi arg='arg2'>x</mi></mrow> </math>";
    test("ru", "SimpleSpeak", expr, "2 икс")?;
    test("ru", "LiteralSpeak", expr, "2 икс")?;
    return Ok(());
}

#[test]
fn prefix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='testing:prefix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("ru", "SimpleSpeak", expr, "testing икс")?;
    return Ok(());
}

#[test]
fn postfix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='testing:postfix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс testing")?;
    return Ok(());
}

#[test]
fn infix_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:infix($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс testing игрек testing зэт testing 2")?;
    return Ok(());
}

#[test]
fn infix_intent_no_args() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:infix()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс")?;
    return Ok(());
}

#[test]
fn infix_intent_one_arg() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:infix($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    // Note: we say the intent name because there are infix plus/minus with a single arg due to continued rows or combined columns
    test("ru", "SimpleSpeak", expr, "testing икс")?;
    return Ok(());
}

#[test]
fn function_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:function($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "testing от икс запятая; игрек запятая, зэт запятая, 2")?;
    return Ok(());
}

#[test]
fn function_no_args_intent() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:function()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс")?;
    return Ok(());
}

#[test]
fn function_one_arg_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:function($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "testing от икс")?;
    return Ok(());
}

#[test]
fn silent_intent_mi() -> Result<()> {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "2")?;
    test("ru", "ClearSpeak", expr, "2")?;
    return Ok(());
}

#[test]
fn silent_intent_msup() -> Result<()> {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("ru", "SimpleSpeak", expr, "заглавная аш 2")?;
    test("ru", "ClearSpeak", expr, "заглавная аш 2")?;
    return Ok(());
}

#[test]
fn silent_intent_underscore() -> Result<()> {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("ru", "SimpleSpeak", expr, "заглавная аш 2")?;
    test("ru", "ClearSpeak", expr, "заглавная аш 2")?;
    return Ok(());
}

#[test]
fn intent_prob_x() -> Result<()> {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='probability' mathvariant='normal'>P</mi>
    </msup></math>";
    // An indirectly referenced W3C name keeps its Russian noun grammar in every speech mode.
    all_russian_speech_modes(expr, "вероятность икс")?;
    return Ok(());
}

// Reviewed W3C names and argument-grammar regressions.
#[test]
fn w3c_core_translations() -> Result<()> {
    // Each previously missing core name has a reviewed speech contract in both styles and all verbosities.
    // Inventory source: https://w3c.github.io/mathml-docs/intent-core-concepts/ (reviewed 2026-10-30).
    let mut count=0;
    for line in include_str!("w3c_core_expected.tsv").lines().skip(1) {
        let fields: Vec<_>=line.splitn(4,'\t').collect();
        let name=fields[0];
        let arity: usize=fields[1].parse()?;
        let references=(0..arity).map(|i|format!("$a{i}")).collect::<Vec<_>>().join(",");
        let children=(0..arity).map(|i|format!("<mi arg='a{i}'>{}</mi>",['x','y','z','n'][i%4])).collect::<String>();
        let mathml=if arity==0 {format!("<math><mi intent='{name}'>x</mi></math>")} else {format!("<math><mrow intent='{name}({references})'>{children}</mrow></math>")};
        for style in ["ClearSpeak", "SimpleSpeak"] {
            for verbosity in ["Terse", "Medium", "Verbose"] {
                // Existing generic function speech omits 'от' in terse mode.
                let expected=if verbosity=="Terse" {fields[2]} else {fields[3]};
                test_prefs("ru",style,vec![("Verbosity",verbosity),("IntentErrorRecovery","Error")],&mathml,expected)?;
            }
        }
        count+=1;
    }
    assert_eq!(count,169,"The audited core inventory must not silently lose cases");
    Ok(())
}

#[test]
fn w3c_logarithm_argument_and_base() -> Result<()> {
    // W3C orders arguments as value, base; Russian speech says 'по основанию base от value'.
    all_russian_speech_modes("<math><mrow intent='logarithm($value,$base)'><msub><mi>log</mi><mn arg='base'>2</mn></msub><mn arg='value'>8</mn></mrow></math>","логарифм по основанию 2 от 8")?;
    for style in ["ClearSpeak", "SimpleSpeak"] {
        for verbosity in ["Terse", "Medium", "Verbose"] {
            let expected=if verbosity=="Terse" {"логарифм 8"} else {"логарифм от 8"};
            test_prefs("ru",style,vec![("Verbosity",verbosity)],"<math><mrow intent='logarithm(8)'/></math>",expected)?;
        }
    }
    Ok(())
}

#[test]
fn w3c_sum_arities() -> Result<()> {
    // Domain/bound arguments precede the summand in the two- and three-argument W3C forms.
    all_russian_speech_modes("<math><mrow intent='sum(2)'/></math>","сумма 2")?;
    all_russian_speech_modes("<math><mrow intent='sum(2,3)'/></math>","сумма по 2 выражения 3")?;
    all_russian_speech_modes("<math><mrow intent='sum(2,3,5)'/></math>","сумма от 2 до 3 выражения 5")
}

#[test]
fn w3c_product_arities() -> Result<()> {
    // The last argument is the factor expression, never an upper bound or an extra factor.
    all_russian_speech_modes("<math><mrow intent='product(2)'/></math>","произведение 2")?;
    all_russian_speech_modes("<math><mrow intent='product(2,3)'/></math>","произведение по 2 выражения 3")?;
    all_russian_speech_modes("<math><mrow intent='product(2,3,5)'/></math>","произведение от 2 до 3 выражения 5")
}

#[test]
fn w3c_partial_derivative_mixed_orders() -> Result<()> {
    // Every variable stays paired with its own order, including an implicit first-order variable.
    all_russian_speech_modes("<math><mrow intent='partial-derivative($f,$x,2,$y,1)'><mi arg='f'>f</mi><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>","частная производная эф по икс порядка 2 и по игрек")?;
    all_russian_speech_modes("<math><mrow intent='partial-derivative($f,x,$n,y,3)'><mi arg='f'>f</mi><mi arg='n'>n</mi></mrow></math>","частная производная эф по икс порядка эн и по игрек порядка 3")
}

#[test]
fn w3c_variadic_lists_and_coordinates() -> Result<()> {
    // Every entry/coordinate of a longer list must be spoken once; arity-zero names are in the inventory test.
    all_russian_speech_modes("<math><mrow intent='list(1,2,3)'/></math>","список 1 запятая 2 запятая 3")?;
    all_russian_speech_modes("<math><mrow intent='tuple(1,2,3)'/></math>","кортеж 1 запятая 2 запятая 3")?;
    all_russian_speech_modes("<math><mrow intent='cartesian-coordinate(1,2,3,4)'/></math>","декартовы координаты 1 запятая 2 запятая 3 запятая 4")
}

#[test]
fn w3c_variadic_extrema() -> Result<()> {
    // max/min accept more than one value and must not drop the final value.
    all_russian_speech_modes("<math><mrow intent='max(1,2,3)'/></math>","максимум из 1 запятая 2 запятая 3")?;
    all_russian_speech_modes("<math><mrow intent='min(1,2,3)'/></math>","минимум из 1 запятая 2 запятая 3")
}

#[test]
fn w3c_nested_noun_cases() -> Result<()> {
    // Nested noun heads take genitive forms, in either order of old and newly supported concepts.
    all_russian_speech_modes("<math><mrow intent='cardinality(image($r))'><mi>Im</mi><mi arg='r'>R</mi></mrow></math>","мощность образа заглавной эр")?;
    all_russian_speech_modes("<math><mrow intent='image(cardinality($r))'><mi>Im</mi><mi arg='r'>R</mi></mrow></math>","образ мощности заглавной эр")?;
    all_russian_speech_modes("<math><mrow intent='mean(list(1,2,3))'/></math>","среднее списка 1 запятая 2 запятая 3")
}

#[test]
fn w3c_division_and_condition_argument_order() -> Result<()> {
    // Non-symmetric numeric operands expose accidental reversal of dividend/divisor or event/condition.
    all_russian_speech_modes("<math><mrow intent='quotient(7,3)'/></math>","целая часть результата деления 7 на 3")?;
    all_russian_speech_modes("<math><mrow intent='remainder(7,3)'/></math>","остаток от деления 7 на 3")?;
    all_russian_speech_modes("<math><mrow intent='ratio(7,3)'/></math>","отношение 7 к 3")?;
    all_russian_speech_modes("<math><mrow intent='conditional-probability(7,3)'/></math>","вероятность 7 при условии 3")
}

#[test]
fn w3c_relation_and_operation_cases() -> Result<()> {
    // Check grammatical words independently of automatically generated pauses.
    for style in ["ClearSpeak","SimpleSpeak"] {
        for (name,operator,expected) in [
            ("equals","=","заглавная ку равно заглавной эр"),
            ("element-of","∈","заглавная ку принадлежит заглавной эр"),
            ("times","×","заглавная ку умножить на заглавную эр"),
            ("implies","⇒","заглавная ку влечёт заглавную эр"),
        ] {
            let mathml=format!("<math><mrow intent='{name}($q,$r)'><mi arg='q'>Q</mi><mo>{operator}</mo><mi arg='r'>R</mi></mrow></math>");
            init_nav("ru",&mathml)?;
            set_preference("SpeechStyle",style)?;
            set_preference("SpeechOverrides_CapitalLetters","")?;
            set_preference("Impairment","Blindness")?;
            let actual=get_spoken_text()?.replace([',',';'],"").split_whitespace().collect::<Vec<_>>().join(" ");
            assert_eq!(actual,expected,"{style}/{name}");
        }
    }
    Ok(())
}

#[test]
fn w3c_read_named_logarithm_and_relation() -> Result<()> {
    // Semantic ReadCurrent must retain both argument order and Russian relation wording.
    for style in ["ClearSpeak","SimpleSpeak"] {
        init_nav("ru","<math><mrow intent='logarithm(8,2)'/></math>")?;
        set_preference("SpeechStyle",style)?;
        let speech=do_navigate_command("ReadCurrent")?.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(speech.contains("логарифм по основанию 2 от 8"),"{style}: {speech}");
        init_nav("ru","<math><mrow intent='equals(2,3)'/></math>")?;
        set_preference("SpeechStyle",style)?;
        let speech=do_navigate_command("ReadCurrent")?.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(speech.contains("2 равно 3"),"{style}: {speech}");
    }
    Ok(())
}

use super::*;
use std::collections::BTreeSet;

fn sorted_unique_keys(mut keys: Vec<(String, String)>) -> Vec<(String, String)> {
    keys.sort();
    keys.dedup();
    keys
}

#[test]
fn registered_stdlib_functions_have_exactly_one_doc() {
    let registered = sorted_unique_keys(
        registered_stdlib_function_registrations()
            .iter()
            .map(|registration| {
                (
                    registration.module.to_string(),
                    registration.name.to_string(),
                )
            })
            .collect(),
    );
    let documented = sorted_unique_keys(
        stdlib_function_docs()
            .into_iter()
            .map(|doc| (doc.module, doc.name))
            .collect(),
    );

    let missing_docs: Vec<_> = registered
        .iter()
        .filter(|key| !documented.contains(key))
        .collect();
    let stale_docs: Vec<_> = documented
        .iter()
        .filter(|key| !registered.contains(key))
        .collect();

    assert!(
        missing_docs.is_empty() && stale_docs.is_empty(),
        "stdlib doc mismatch: missing_docs={missing_docs:?}, stale_docs={stale_docs:?}"
    );
}

#[test]
fn every_registered_stdlib_function_has_runtime_context_metadata() {
    let docs = stdlib_function_docs();
    assert_eq!(docs.len(), registered_stdlib_function_registrations().len());
    for doc in &docs {
        assert_eq!(
            stdlib_function_context(&doc.module, &doc.name),
            Some(doc.context),
            "context lookup differs for {}::{}",
            doc.module,
            doc.name
        );
    }
    assert_eq!(stdlib_function_context("missing", "function"), None);
    assert_eq!(
        find_stdlib_function_doc("combat", "use_skill")
            .expect("combat::use_skill should be documented")
            .context,
        StdlibFunctionContext::ActiveCombat
    );
    assert_eq!(
        find_stdlib_function_doc("combat", "try_use_skill_and_wait")
            .expect("combat::try_use_skill_and_wait should be documented")
            .context,
        StdlibFunctionContext::CombatActiveOrTerminal
    );
    assert_eq!(
        find_stdlib_function_doc("combat", "get_result")
            .expect("combat::get_result should be documented")
            .context,
        StdlibFunctionContext::OutOfCombat
    );
    assert_eq!(
        find_stdlib_function_doc("combat", "get_action_snapshot")
            .expect("combat::get_action_snapshot should be documented")
            .context,
        StdlibFunctionContext::Any
    );
}

#[test]
fn every_registered_stdlib_function_has_explicit_details() {
    let details = super::docs::detailed_stdlib_function_details_by_key();
    let registered = registered_stdlib_function_registrations()
        .iter()
        .map(|registration| registration.key())
        .collect::<BTreeSet<_>>();
    let documented = details.keys().copied().collect::<BTreeSet<_>>();

    assert_eq!(documented, registered);
}

#[test]
fn stdlib_function_docs_do_not_contain_duplicates() {
    let docs = stdlib_function_docs()
        .into_iter()
        .map(|doc| (doc.module, doc.name))
        .collect::<Vec<_>>();
    let unique_docs = sorted_unique_keys(docs.clone());

    assert_eq!(docs.len(), unique_docs.len(), "duplicate stdlib docs found");
}

#[test]
fn submission_functions_use_submit_prefix() {
    let offenders = registered_stdlib_function_registrations()
        .iter()
        .filter(|registration| {
            let name = registration.name;
            name.contains("settle")
                || name.contains("report_fight")
                || name.contains("report_battle_win")
                || name.contains("commit")
                || (name.contains("submit")
                    && (name.contains("_fight")
                        || name.contains("_battle")
                        || (name != "submit"
                            && !name.starts_with("submit_")
                            && !name.starts_with("try_submit_"))))
        })
        .map(|registration| format!("{}::{}", registration.module, registration.name))
        .collect::<Vec<_>>();

    assert!(
        offenders.is_empty(),
        "script submission functions must use a submit prefix: {offenders:?}"
    );
}

#[test]
fn stdlib_function_names_do_not_repeat_their_namespace_or_old_spellings() {
    let offenders = registered_stdlib_function_registrations()
        .iter()
        .filter(|registration| {
            let name = registration.name;
            let repeats_namespace = match registration.module {
                "lookup" => name.starts_with("lookup_"),
                "session" => name.starts_with("session_"),
                "combat" => matches!(
                    name,
                    "combat_escape"
                        | "get_battle_history"
                        | "get_battle_result"
                        | "get_combat_actions"
                        | "get_combat_lineup"
                        | "get_combat_state"
                        | "is_combat_finished"
                        | "try_combat_escape"
                        | "try_combat_escape_and_wait"
                        | "try_get_battle_result"
                ),
                _ => false,
            };
            let old_spelling = name == "begin"
                || name.starts_with("begin_")
                || name.contains("minigame")
                || name.contains("onekey")
                || name.contains("growup")
                || name.contains("_exc_")
                || name.contains("fight")
                || name.contains("battle")
                || name.contains("_get_gift")
                || name.contains("_get_reward")
                || name == "get_prize"
                || name == "get_gift"
                || name == "get_floor_award"
                || name == "get_top_reward";
            repeats_namespace || old_spelling
        })
        .map(|registration| format!("{}::{}", registration.module, registration.name))
        .collect::<Vec<_>>();

    assert!(
        offenders.is_empty(),
        "stdlib functions contain redundant namespaces or old spellings: {offenders:?}"
    );
}

#[test]
fn stdlib_function_docs_do_not_expose_placeholder_copy() {
    let placeholders = ["待补充", "取决于具体接口", "详细参数语义"];
    let offenders = stdlib_function_docs()
        .into_iter()
        .filter(|doc| {
            [&doc.description, &doc.returns].into_iter().any(|text| {
                placeholders
                    .iter()
                    .any(|placeholder| text.contains(placeholder))
            }) || doc.params.iter().any(|param| {
                placeholders
                    .iter()
                    .any(|placeholder| param.description.contains(placeholder))
            })
        })
        .map(|doc| format!("{}::{}", doc.module, doc.name))
        .collect::<Vec<_>>();

    assert!(
        offenders.is_empty(),
        "placeholder stdlib docs found: {offenders:?}"
    );
}

#[test]
fn registration_parameter_names_handle_empty_and_variadic_signatures() {
    assert_eq!(
        StdlibFunctionRegistration::new("demo", "empty", "demo::empty()").parameter_names(),
        Vec::<String>::new()
    );
    assert_eq!(
        StdlibFunctionRegistration::new("demo", "variadic", "demo::variadic(...)")
            .parameter_names(),
        Vec::<String>::new()
    );
}

#[test]
fn return_docs_are_inferred_from_registered_types() {
    let docs = stdlib_function_docs();
    let scene_spirits = docs
        .iter()
        .find(|doc| doc.module == "scene" && doc.name == "get_scene_spirits")
        .expect("scene::get_scene_spirits should be documented");

    let return_doc = scene_spirits
        .return_doc
        .as_ref()
        .expect("SceneSpiritInfo[] should infer return doc");
    assert_eq!(return_doc.type_name, "SceneSpiritInfo[]");
    assert!(return_doc
        .fields
        .iter()
        .any(|field| field.name == "spirit_id"));
}

#[test]
fn pet_egg_functions_expose_result_struct_docs() {
    let docs = stdlib_function_docs();
    let expected = [
        ("query_info", "PetEggInfo"),
        ("vip_speed_up", "PetEggSpeedUpResult"),
        ("start", "PetEggBeginResult"),
        ("cancel", "PetEggCancelResult"),
        ("preview", "PetEggPreviewResult"),
    ];

    for (name, return_type) in expected {
        let doc = docs
            .iter()
            .find(|doc| doc.module == "pet_egg" && doc.name == name)
            .unwrap_or_else(|| panic!("pet_egg::{name} should be documented"));
        let return_doc = doc
            .return_doc
            .as_ref()
            .unwrap_or_else(|| panic!("pet_egg::{name} should expose return docs"));
        assert_eq!(return_doc.type_name, return_type);
        assert!(!return_doc.fields.is_empty());
    }
}

#[test]
fn generated_rust_return_types_cover_pet_egg_registration() {
    for (name, return_type) in [
        ("query_info", "PetEggInfo"),
        ("vip_speed_up", "PetEggSpeedUpResult"),
        ("start", "PetEggBeginResult"),
        ("cancel", "PetEggCancelResult"),
        ("preview", "PetEggPreviewResult"),
    ] {
        assert_eq!(
            super::generated_stdlib_return_type("pet_egg", name),
            Some(return_type),
            "missing generated Rust return type for pet_egg::{name}"
        );
    }
}

#[test]
fn type_docs_include_nested_types_without_direct_function_returns() {
    let docs = stdlib_type_docs();
    assert!(docs
        .iter()
        .any(|doc| doc.type_name == "StaticSpiritEvolutionEdge"));
}

#[test]
fn reachable_struct_fields_have_type_docs() {
    let docs = stdlib_type_docs();
    let known = docs
        .iter()
        .map(|doc| doc.type_name.as_str())
        .collect::<std::collections::HashSet<_>>();
    let primitives = [
        "()", "bool", "int", "float", "char", "string", "dynamic", "map", "Map", "blob",
    ];
    let missing = docs
        .iter()
        .flat_map(|doc| &doc.fields)
        .filter_map(|field| {
            let type_name = field
                .type_name
                .split('|')
                .next()
                .unwrap_or_default()
                .trim()
                .trim_end_matches('?')
                .trim_end_matches("[]");
            (!type_name.is_empty()
                && !primitives.contains(&type_name)
                && !known.contains(type_name))
            .then_some(type_name)
        })
        .collect::<std::collections::BTreeSet<_>>();

    assert!(
        missing.is_empty(),
        "missing reachable type docs: {missing:?}"
    );
}

#[test]
fn documented_struct_returns_have_return_docs() {
    let primitive_returns = ["()", "bool", "int", "string", "map", "Array", "blob"];
    let docs = stdlib_function_docs();
    let missing: Vec<_> = docs
        .iter()
        .filter_map(|doc| {
            let return_type = types::infer_return_type(&doc.signature)?;
            let normalized = return_type.trim_end_matches("[]");
            if primitive_returns.contains(&normalized) {
                return None;
            }
            doc.return_doc
                .is_none()
                .then_some(format!("{}::{} -> {}", doc.module, doc.name, return_type))
        })
        .collect();

    assert!(
        missing.is_empty(),
        "missing return docs for struct returns: {missing:?}"
    );
}

#[test]
fn stdlib_docs_do_not_contain_mojibake_or_replacement_text() {
    let docs = stdlib_function_docs();
    let mut bad = Vec::new();
    for doc in docs {
        for value in [
            doc.description.as_str(),
            doc.returns.as_str(),
            doc.signature.as_str(),
        ] {
            if looks_corrupted(value) {
                bad.push(format!("{}::{}: {}", doc.module, doc.name, value));
            }
        }
        for param in &doc.params {
            if looks_corrupted(&param.description) {
                bad.push(format!(
                    "{}::{} param {}: {}",
                    doc.module, doc.name, param.name, param.description
                ));
            }
        }
        if let Some(return_doc) = &doc.return_doc {
            if looks_corrupted(&return_doc.description) {
                bad.push(format!(
                    "{}::{} return: {}",
                    doc.module, doc.name, return_doc.description
                ));
            }
            for field in &return_doc.fields {
                if looks_corrupted(&field.description) {
                    bad.push(format!(
                        "{}::{} field {}: {}",
                        doc.module, doc.name, field.name, field.description
                    ));
                }
            }
        }
    }

    assert!(bad.is_empty(), "corrupted stdlib docs found: {bad:?}");
}

#[test]
fn stdlib_docs_expose_namespace_labels_without_removed_compatibility_apis() {
    let docs = stdlib_function_docs();
    assert!(docs.iter().all(|doc| !doc.module_label.trim().is_empty()));

    let reclaim_docs = docs
        .iter()
        .filter(|doc| doc.module == "reclaim_goods")
        .collect::<Vec<_>>();
    assert!(!reclaim_docs.is_empty());
    assert!(reclaim_docs
        .iter()
        .all(|doc| doc.module_label == "物品回收命令"));
    assert!(reclaim_docs
        .iter()
        .all(|doc| !doc.name.ends_with("with_safe_code")));
}

fn looks_corrupted(value: &str) -> bool {
    value.contains("????") || value.contains('�') || value.contains('鍙') || value.contains('杩')
}

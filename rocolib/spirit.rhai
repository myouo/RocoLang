fn format_skill_requirements(required_skill_ids) {
    const skill_infos = lookup::skill_infos(required_skill_ids);
    let text = "";
    let is_first = true;
    for skill_info in skill_infos {
        if is_first {
            is_first = false;
        } else {
            text = text + ",";
        }
        text = text + skill_info.name + "(" + skill_info.id + ")";
    }

    return "[" + text + "]";
}

fn is_spirit_ready(spirit_info, spirit_id, required_skill_ids) {
    if spirit_info.spirit_id != spirit_id {
        return #{
            ok: false,
            code: 1,
            message: "spirit id mismatch: expected " + spirit_id + ", got " + spirit_info.spirit_id
        };
    }

    let missing_skill_ids = [];
    for required_skill_id in required_skill_ids {
        let found = false;
        for skill in spirit_info.skills {
            if skill.skill_id == required_skill_id {
                found = true;
            }
        }

        if !found {
            missing_skill_ids.push(required_skill_id);
        }
    }

    if len(missing_skill_ids) != 0 {
        return #{
            ok: false,
            code: 2,
            message: "spirit " + spirit_id + " missing required skills " + format_skill_requirements(missing_skill_ids)
        };
    }

    return #{
        ok: true,
        code: 0,
        message: "spirit " + spirit_id + " is ready"
    };
}

fn select_best_storage_spirit(spirit_id, required_skill_ids) {
    const storage_spirits = spirit::list_storage_spirits();
    let best = ();
    let found = false;
    for spirit_info in storage_spirits {
        if spirit_info.spirit_id == spirit_id {
            let has_required_skills = len(required_skill_ids) == 0;
            if spirit_info.level == 100 {
                const detail = spirit::get_storage_spirit_detail(spirit_info.spirit_id, spirit_info.catch_time);
                has_required_skills = is_spirit_ready(detail, spirit_id, required_skill_ids).ok;
            }

            if !found {
                best = #{
                    spirit_id: spirit_info.spirit_id,
                    catch_time: spirit_info.catch_time,
                    level: spirit_info.level,
                    storage_time: spirit_info.storage_time,
                    has_required_skills: has_required_skills
                };
                found = true;
            } else if spirit_info.level > best.level {
                best = #{
                    spirit_id: spirit_info.spirit_id,
                    catch_time: spirit_info.catch_time,
                    level: spirit_info.level,
                    storage_time: spirit_info.storage_time,
                    has_required_skills: has_required_skills
                };
            } else if spirit_info.level == best.level && spirit_info.level == 100 && has_required_skills && !best.has_required_skills {
                best = #{
                    spirit_id: spirit_info.spirit_id,
                    catch_time: spirit_info.catch_time,
                    level: spirit_info.level,
                    storage_time: spirit_info.storage_time,
                    has_required_skills: has_required_skills
                };
            } else if spirit_info.level == best.level && has_required_skills == best.has_required_skills && spirit_info.storage_time > best.storage_time {
                best = #{
                    spirit_id: spirit_info.spirit_id,
                    catch_time: spirit_info.catch_time,
                    level: spirit_info.level,
                    storage_time: spirit_info.storage_time,
                    has_required_skills: has_required_skills
                };
            }
        }
    }

    if !found {
        return #{ found: false };
    }

    return #{
        found: true,
        spirit_id: best.spirit_id,
        catch_time: best.catch_time,
        level: best.level,
        storage_time: best.storage_time,
        has_required_skills: best.has_required_skills
    };
}

fn try_recover_all_spirits_by_mode(recovery_mode, is_vip) {
    if recovery_mode == "vip" {
        return spirit::try_recover_all_spirits();
    }

    if recovery_mode == "hospital" {
        return try_recover_all_spirits_hospital();
    }

    if recovery_mode == "low-frequency-hospital" {
        return try_recover_all_spirits_low_frequency_hospital();
    }

    if recovery_mode == "money" {
        return try_recover_all_spirits_money();
    }

    return try_auto_recover_all_spirits(is_vip);
}

fn try_recover_all_spirits_low_frequency_hospital() {
    const RECOVERY_THRESHOLD_BASIS_POINTS = 2000;
    const bag = spirit::get_spirit_bag();
    const spirit_count = len(bag.spirits);
    if spirit_count == 0 {
        return #{ ok: true, code: 0, message: "empty bag; skipped low-frequency hospital recovery" };
    }

    let skill_ids = [];
    for spirit_info in bag.spirits {
        for skill in spirit_info.skills {
            skill_ids.push(skill.skill_id);
        }
    }
    const skill_infos = lookup::skill_infos(skill_ids);
    const average_basis_points = low_frequency_team_power_pp_basis_points(
        bag.spirits,
        skill_infos
    );

    for spirit_info in bag.spirits {
        const spirit_basis_points = low_frequency_spirit_power_pp_basis_points(
            spirit_info,
            skill_infos
        );
        print(
            "low-frequency hospital PP: position="
                + spirit_info.position
                + " hp="
                + spirit_info.hp
                + "/"
                + spirit_info.max_hp
                + " power_pp="
                + spirit_basis_points / 100
                + "%"
        );
    }

    print(
        "low-frequency hospital team power PP="
            + average_basis_points / 100
            + "% threshold="
            + RECOVERY_THRESHOLD_BASIS_POINTS / 100
            + "%"
    );
    if average_basis_points >= RECOVERY_THRESHOLD_BASIS_POINTS {
        return #{ ok: true, code: 0, message: "power skill PP is sufficient; skipped hospital recovery" };
    }

    return try_recover_all_spirits_hospital();
}

fn low_frequency_team_power_pp_basis_points(spirits, skill_infos) {
    if len(spirits) == 0 {
        return 10000;
    }
    let total_basis_points = 0;
    for spirit_info in spirits {
        total_basis_points = total_basis_points
            + low_frequency_spirit_power_pp_basis_points(spirit_info, skill_infos);
    }
    return total_basis_points / len(spirits);
}

fn low_frequency_spirit_power_pp_basis_points(spirit_info, skill_infos) {
    if spirit_info.hp <= 0 {
        return 0;
    }

    let current_power_pp = 0;
    let maximum_power_pp = 0;
    for skill in spirit_info.skills {
        const skill_info = find_static_skill_info(skill_infos, skill.skill_id);
        if skill_info.found && is_power_skill_value(skill_info.power) {
            let maximum_pp = skill.max_pp;
            if maximum_pp <= 0 {
                maximum_pp = skill_info.pp_max;
            }
            if maximum_pp > 0 {
                let current_pp = skill.pp;
                if current_pp < 0 {
                    current_pp = 0;
                } else if current_pp > maximum_pp {
                    current_pp = maximum_pp;
                }
                current_power_pp = current_power_pp + current_pp;
                maximum_power_pp = maximum_power_pp + maximum_pp;
            }
        }
    }
    if maximum_power_pp == 0 {
        return 0;
    }
    return current_power_pp * 10000 / maximum_power_pp;
}

fn find_static_skill_info(skill_infos, skill_id) {
    for skill_info in skill_infos {
        if skill_info.id == skill_id {
            return #{ found: true, power: skill_info.power, pp_max: skill_info.pp_max };
        }
    }
    return #{ found: false, power: "", pp_max: 0 };
}

fn is_power_skill_value(power) {
    return power != "--" && power != "" && power != "0";
}

fn try_auto_recover_all_spirits(is_vip) {
    if is_vip {
        let recover_result = spirit::try_recover_all_spirits();
        if recover_result.ok {
            return recover_result;
        }
        print("vip recovery failed: " + recover_result.message);
    }

    let hospital_result = try_recover_all_spirits_hospital();
    if hospital_result.ok {
        return hospital_result;
    }
    print("hospital recovery failed: " + hospital_result.message);

    return try_recover_all_spirits_money();
}

fn try_recover_all_spirits_hospital() {
    const HOSPITAL_SCENE_ID = 57;
    const MOVE_TIMEOUT_MS = 30000;

    let current_scene = scene::get_current_scene();
    if current_scene == HOSPITAL_SCENE_ID {
        return spirit::try_recover_all_spirits_at_hospital();
    }

    let move_result = scene::try_move_to_scene(HOSPITAL_SCENE_ID, MOVE_TIMEOUT_MS);
    if !move_result.ok {
        return move_result;
    }

    let recover_result = spirit::try_recover_all_spirits_at_hospital();
    let restore_result = scene::try_move_to_scene(current_scene, MOVE_TIMEOUT_MS);
    if !restore_result.ok {
        return #{
            ok: false,
            code: restore_result.code,
            message: "failed to restore original scene after hospital recovery: " + restore_result.message
        };
    }

    return recover_result;
}

fn try_recover_all_spirits_money() {
    let bag = spirit::get_spirit_bag();
    let restored_count = 0;
    for spirit_info in bag.spirits {
        if spirit_info.hp < spirit_info.max_hp {
            let restore_result = spirit::try_restore_spirit(spirit_info.spirit_id, spirit_info.position);
            if !restore_result.ok {
                return #{
                    ok: false,
                    code: restore_result.code,
                    message: "failed to restore spirit at bag position " + spirit_info.position + ": " + restore_result.message + ", restored_count=" + restored_count
                };
            }
            restored_count = restored_count + 1;
        }
    }

    return #{
        ok: true,
        code: 0,
        message: "restored " + restored_count + " spirits"
    };
}

fn try_store_all_spirits() {
    const bag = spirit::get_spirit_bag();
    let position = len(bag.spirits);
    let stored_count = 0;

    while position >= 1 {
        const result = spirit::try_store_spirit(position);
        if !result.ok {
            return #{
                ok: false,
                code: result.code,
                message: "failed to store spirit at position " + position + ": " + result.message + ", stored_count=" + stored_count
            };
        }
        stored_count = stored_count + 1;
        position = position - 1;
    }

    const verify_bag = spirit::get_spirit_bag();
    if len(verify_bag.spirits) != 0 {
        return #{
            ok: false,
            code: 1,
            message: "bag still has " + len(verify_bag.spirits) + " spirits after storing " + stored_count
        };
    }

    return #{
        ok: true,
        code: 0,
        message: "stored " + stored_count + " spirits"
    };
}

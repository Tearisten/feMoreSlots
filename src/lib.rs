#![feature(ptr_sub_ptr)]

use unity::prelude::*;
use skyline::install_hook;
use cobapi::{Event, SystemEvent};
use engage::{
    gamedata::skill::*,
    gamedata::unit::Unit,
    gamedata::*
};
use skyline::patching::Patch;
use unity::il2cpp::object::Array;

// EquipSkillMenu
// public static InfoUtil.StatusSkill[] GetSkillListForUnitInfo(Unit unit, bool isSkillEquip = False, bool isPack = False, int size = 10) { }
// public class InfoUtil.StatusSkill // TypeDefIndex: 11851
// {
	// Properties
	// public SkillData Data { get; set; }
	// public bool IsActive { get; set; }
	// public SkillData.Categorys Category { get; set; }


#[unity::class("App", "StringBuilder")]
pub struct StringBuilder{
	pub m_ChunkChars: Array<char>, // 0x10
	pub m_ChunkPrevious: Box<StringBuilder>, // 0x18
	pub m_ChunkLength : i32, // 0x20
	pub m_ChunkOffset : i32, // 0x24
	pub m_MaxCapacity : i32 // 0x28
}

#[unity::class("App", "InfoUtil")]
pub struct InfoUtil{
    pub emptyStringBuilder: StringBuilder
	// private static readonly StringBuilder s_EmptyStringBuilder; // 0x0
}

#[unity::class("App", "InfoUtil_StatusSkill")]
pub struct StatusSkill {
    pub skill_data: &'static SkillData,
    pub is_active: bool,
    pub category: u32
}

#[skyline::from_offset(0x3780700)]
pub fn is_null_empty(this: &Il2CppString, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "Unit", "get_EquipSkill")]
pub fn get_equiped_skills(this: &Unit, _method_info: u64) -> &'static SkillArray;

// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_IsActive")]
#[skyline::from_offset(0x1FC7310)]
pub fn set_is_active(this: &mut StatusSkill, active: bool, _method_info: u64);

// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_Category")]
#[skyline::from_offset(0x1FC7330)]
pub fn set_category(this: &mut StatusSkill, cat: u32, _method_info: u64);

// #[unity::from_offset("App", "InfoUtil_StatusSkill", "set_Data")]
#[skyline::from_offset(0x1FC72F0)]
pub fn set_data(this: &mut StatusSkill, value: &SkillData, _method_info: u64);

#[unity::hook("App", "InfoUtil", "GetSkillListForUnitInfo")]
pub fn get_skill_list(this: &InfoUtil, unit: &Unit, is_equip: bool, is_pack: bool, size: i32, _method_info : u64) -> &'static mut Array<&'static mut StatusSkill>
{
    let mut original = call_original!(this, unit, is_equip, is_pack, size, _method_info);
    unsafe
    {
        
        // skyline::error::show_error(
        //     69,
        //     "Tristen",
        //     "Called yay"
        // );

        if !is_equip && Some(original).is_some() && Some(unit).is_some()
        {
            let equips = &unit.equip_skill;
            // this is the empty skill
            // set_data(original[0], SkillData::try_index_get(0).unwrap(), _method_info);
            // skyline::error::show_error(
            //     69,
            //     "Tristen",
            //     "Called 1"
            // );
            // for x in 0..equips.len()
            // {


            //     // set_data(original[x], equips[x].get_skill().unwrap(), _method_info);
            // }
 
            
            // for x in original.len()-1..4
            // {
            //     original[x] = original[x-1];
            // }
            // for x in 2..5
            // {
            //     // set_is_active(original[x], false, _method_info);
            //     // set_category(original[x], 11, _method_info);
            //     let empty : &'static SkillData;
            //     set_data(original[x], empty, _method_info);
            // }
        }
    }
    return original;
}


#[skyline::main(name = "moreSlots")]
pub fn main() {
    println!("No Unique Weapons plugin loaded");

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    // category on equip screen
    // Patch::in_text(0x0249b27c).bytes(&[0x60, 0x01, 0x80,0x52]).unwrap();

    // chekcs in add skill for max count (5 now)
    // 01a35fd0 && 01a35ff
    Patch::in_text(0x01a35fd0).bytes(&[0x1f, 0x11, 0x00, 0x71]).unwrap();
    Patch::in_text(0x01a35ff8).bytes(&[0x1f, 0x11, 0x00, 0x71]).unwrap();

    // set color change count to 5
    // 0249b3f8 3f 11 00 71
    Patch::in_text(0x0249b3f8).bytes(&[0x3f, 0x11, 0x00, 0x71]).unwrap();

    // set get list arrray size
    //02911058 74 01 80 52
    Patch::in_text(0x02911058).bytes(&[0x74, 0x01, 0x80, 0x52]).unwrap();


    install_hook!(get_skill_list);
}
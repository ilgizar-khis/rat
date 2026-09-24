use std::{cell::RefCell, rc::Rc};

use mlua::{Lua, Result, Table};

use crate::{App, actions::RatActions, rat::Rat};

pub fn create_lua_functions(app: &Rc<RefCell<App>>) -> Result<Lua> {
    let lua = Lua::new();

    let rat = lua.create_table()?;

    let lua_new_rat_app = Rc::clone(app);

    let rat_new = lua.create_function(move |_, opts: Table| {
        let pos: [u16; 2] = opts.get("pos").unwrap_or([0; 2]);
        let size: u16 = opts.get("size").unwrap_or(1);
        let field: [u16; 2] = opts.get("field").unwrap_or([30; 2]);
        let color: String = opts.get("color").unwrap_or("white".to_string());

        let rat = Rat::new(pos, size, field, color);
        let id = lua_new_rat_app.borrow_mut().add_rat(rat);
        Ok(id)
    })?;

    rat.set("new", rat_new)?;

    let lua_move_dir_app = Rc::clone(app);

    let rat_move_dir = lua
        .create_function(move |_, opts: Table| {
            let id: String = opts.get("id")?;
            let dir: String = opts.get("dir")?;
            let times: usize = opts.get("times").unwrap_or(1);
            let mut app = lua_move_dir_app.borrow_mut();
            match app.rats.get_mut(&id) {
                Some(robot) => {
                    robot.add_action(RatActions::MoveDir(dir), times);
                    Ok(())
                }
                None => Err(mlua::Error::RuntimeError(format!(
                    "robot with id '{id}' not found"
                ))),
            }
        })
        .unwrap();
    rat.set("move_dir", rat_move_dir)?;

    let lua_move_to_app = Rc::clone(app);
    let rat_move_to = lua
        .create_function(move |_, opts: Table| {
            let id: String = opts.get("id")?;
            let pos: [u16; 2] = opts.get("pos")?;
            let mut app = lua_move_to_app.borrow_mut();
            match app.rats.get_mut(&id) {
                Some(robot) => {
                    robot.add_action(RatActions::MoveTo(pos), 1);
                    Ok(())
                }
                None => Err(mlua::Error::RuntimeError(format!(
                    "robot with id '{id}' not found"
                ))),
            }
        })
        .unwrap();
    rat.set("move_to", rat_move_to)?;

    let lua_set_color_app = Rc::clone(app);
    let rat_set_color = lua
        .create_function(move |_, opts: Table| {
            let id: String = opts.get("id")?;
            let color: String = opts.get("color")?;
            let mut app = lua_set_color_app.borrow_mut();
            match app.rats.get_mut(&id) {
                Some(robot) => {
                    robot.add_action(RatActions::SetColor(color), 1);
                    Ok(())
                }
                None => Err(mlua::Error::RuntimeError(format!(
                    "robot with id '{id}' not found"
                ))),
            }
        })
        .unwrap();
    rat.set("set_color", rat_set_color)?;

    let lua_set_size_app = Rc::clone(app);
    let rat_set_size = lua
        .create_function(move |_, opts: Table| {
            let id: String = opts.get("id")?;
            let size: u16 = opts.get("size")?;
            let mut app = lua_set_size_app.borrow_mut();
            match app.rats.get_mut(&id) {
                Some(robot) => {
                    robot.add_action(RatActions::SetSize(size), 1);
                    Ok(())
                }
                None => Err(mlua::Error::RuntimeError(format!(
                    "robot with id '{id}' not found"
                ))),
            }
        })
        .unwrap();
    rat.set("set_size", rat_set_size)?;

    lua.globals().set("rat", rat)?;

    Ok(lua)
}

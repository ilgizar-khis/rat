use std::{cell::RefCell, rc::Rc};

use mlua::{Lua, Result, Table};

use crate::{App, rat::Rat};

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
    let lua_move_to_app = Rc::clone(app);
    let lua_set_color_app = Rc::clone(app);
    let lua_set_size_app = Rc::clone(app);

    lua.globals().set("rat", rat)?;

    Ok(lua)
}

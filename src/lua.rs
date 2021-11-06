use mlua::prelude::*;

pub fn run() -> LuaResult<()> {
    let lua = Lua::new();

    let map_table1 = lua.create_table()?;
    map_table1.set(1, "one")?;
    map_table1.set("two", 2)?;
    map_table1.set(4, 4)?;


    let map_table2 = lua.create_table()?;
    map_table2.set(1, "one")?;
    map_table2.set("two", 2)?;
    map_table2.set(4, 4)?;

    lua.globals().set("map_table1", map_table1)?;
    lua.globals().set("map_table2", map_table2)?;

    lua.load("
        for k1,v1 in pairs(map_table1) do 
            for k2,v2 in pairs(map_table2) do 
                print(k1,v1,k2,v2)
            end
        end
    ").exec()?;

    map_table2.

    Ok(())
}
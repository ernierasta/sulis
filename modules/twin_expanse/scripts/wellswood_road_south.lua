function disable_go_too_far()
  game:disable_trigger_at(3, 27)
end

function on_add_aessa(parent, target)
  game:add_party_member("npc_aessa")
  disable_go_too_far()
end

function on_add_jorzal(parent, target)
  game:add_party_member("npc_jorzal")
  disable_go_too_far()
end

function on_add_grazi(parent, target)
  game:add_party_member("npc_grazi")
  disable_go_too_far()
end

function on_player_enter_bridge(parent, target)
  game:cancel_blocking_anims()
  game:spawn_encounter_at(17, 36)
  game:enable_trigger_at(35, 65)
end

function on_player_go_too_far(parent, target)
  game:say_line("I should check back with the others before going any further.", parent)
  game:cancel_blocking_anims()
end

function on_player_return(parent, target)
  game:spawn_encounter_at(49, 81)
end

function on_area_load(parent)
  target = game:entity_with_id("npc_tervald")
  
  base_class = game:player():base_class()
  if base_class ~= "fighter" then
    target:set_flag("jorzal_valid_pick")
  end
  
  if base_class ~= "rogue" then
    target:set_flag("grazi_valid_pick")
  end
  
  if base_class ~= "mage" then
    target:set_flag("aessa_valid_pick")
  end
  
  game:start_conversation("tervald", target)
end

function on_ambush_cleared(parent)
  target = game:entity_with_id("npc_tervald")
  target:set_flag("ambush_cleared")
end

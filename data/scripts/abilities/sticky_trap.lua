function on_activate(parent, ability)
  if game:num_effects_with_tag("trap") > 4 then
    game:say_line("Maximum number of traps set.", parent)
    return
  end

  targets = parent:targets()
  
  targeter = parent:create_targeter(ability)
  targeter:set_free_select(1.0)
  targeter:set_free_select_must_be_passable("1by1")
  targeter:set_shape_object_size("1by1")
  targeter:add_all_effectable(targets)
  targeter:activate()
end

function on_target_select(parent, ability, targets)
  points = targets:affected_points()

  surf = parent:create_surface(ability:name(), points)
  surf:set_tag("trap")
  surf:set_squares_to_fire_on_moved(1)
  
  cb = ability:create_callback(parent)
  cb:set_on_moved_in_surface_fn("on_entered")
  surf:add_callback(cb)
  
  anim = parent:create_anim("particles/sticky_trap")
  anim:set_position(anim:param(0.0), anim:param(0.0))
  anim:set_particle_size_dist(anim:fixed_dist(1.0), anim:fixed_dist(1.0))
  anim:set_draw_below_entities()
  surf:add_anim(anim)
  
  surf:apply()
  ability:activate(parent)
end

function on_entered(parent, ability, targets)
  -- only fire on hostiles
  if targets:hostile():is_empty() then return end
  
  targets:surface():mark_for_removal()
  
  target = targets:first()
  hit = parent:special_attack(target, "Reflex", "Ranged")
  duration = 2
  if hit:is_miss() then
    return
  elseif hit:is_graze() then
    duration = duration / 2
  elseif hit:is_hit() then
    -- do nothing
  elseif hit:is_crit() then
    duration = duration * 3 / 2
  end
  
  effect = target:create_effect(ability:name(), duration)
  effect:add_move_disabled()
  
  anim = target:create_particle_generator("particles/circle4")
  anim:set_moves_with_parent()
  anim:set_initial_gen(10.0)
  anim:set_color(anim:param(0.0), anim:param(1.0), anim:param(0.2))
  anim:set_gen_rate(anim:param(20.0))
  anim:set_position(anim:param(0.0), anim:param(0.5))
  anim:set_particle_size_dist(anim:fixed_dist(0.3), anim:fixed_dist(0.3))
  anim:set_particle_position_dist(anim:dist_param(anim:uniform_dist(-0.5, 0.5), anim:uniform_dist(-1.0, 1.0)),
    anim:dist_param(anim:uniform_dist(-0.2, 0.2), anim:uniform_dist(-1.0, 1.0)))
  anim:set_particle_duration_dist(anim:fixed_dist(0.3))
  effect:add_anim(anim)
  effect:apply()
end
line_len = 30.0

function on_activate(parent, ability)
  stats = parent:stats()
  if not stats.attack_is_ranged then
    game:say_line("You must have a ranged weapon equipped.", parent)
    return
  end

  targets = parent:targets():visible():without_self()
  
  targeter = parent:create_targeter(ability)
  targeter:set_free_select(line_len)
  targeter:set_shape_line("1by1", parent:x(), parent:y(), line_len)
  targeter:add_all_effectable(targets)
  targeter:activate()
end

function on_target_select(parent, ability, targets)
  pos = targets:selected_point()
  
  speed = 500 * game:anim_base_time()
  duration = line_len / speed
  
  anim = parent:create_anim(parent:stats().ranged_projectile, duration)

  delta_x = pos.x - parent:x()
  delta_y = pos.y - parent:y()
  angle = game:atan2(delta_x, delta_y)
  
  norm = math.sqrt((delta_x * delta_x) + (delta_y * delta_y))
  
  delta_x = delta_x / norm * line_len
  delta_y = delta_y / norm * line_len
  
  anim:set_position(anim:param(parent:x(), delta_x / duration), anim:param(parent:y(), delta_y / duration))
  anim:set_particle_size_dist(anim:fixed_dist(3.0), anim:fixed_dist(3.0))
  anim:set_rotation(anim:param(angle))
  anim:set_color(anim:param(1.0), anim:param(0.0), anim:param(0.0))
  
  targets = targets:to_table()
  for i = 1, #targets do 
    dist = parent:dist_to_entity(targets[i])
    duration = dist / speed
    
    cb = ability:create_callback(parent)
	cb:add_target(targets[i])
	cb:set_on_anim_update_fn("attack_target")
    anim:add_callback(cb, duration)
  end
  
  anim:activate()
  ability:activate(parent)
end

function attack_target(parent, ability, target)
  target = targets:first()

  stats = parent:stats()
  
  if target:is_valid() then
    parent:special_attack(target, "Reflex", "Ranged", stats.damage_min_0, stats.damage_max_0, stats.armor_penetration_0, "Raw")
  end
end


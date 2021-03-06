function on_activate(parent, ability)
  targets = parent:targets():hostile():visible()
  
  targeter = parent:create_targeter(ability)
  targeter:add_all_selectable(targets)
  targeter:add_all_effectable(targets)
  targeter:activate()
end

function on_target_select(parent, ability, targets)
  stats = parent:stats()
  ability:activate(parent)

  target = targets:first()
  
  hit = parent:special_attack(target, "Will", "Spell")
  amount = -(2 + stats.intellect_bonus / 20) * game:ap_display_factor()
  if hit:is_miss() then
    return
  elseif hit:is_graze() then
    amount = amount / 2
  elseif hit:is_hit() then
    -- do nothing
  elseif hit:is_crit() then
    amount = amount * 1.5
  end
  
  effect = target:create_effect(ability:name(), ability:duration())
  effect:set_tag("slow")
  effect:add_num_bonus("ap", amount)
  
  gen = target:create_anim("slow")
  gen:set_moves_with_parent()
  gen:set_position(gen:param(-0.5), gen:param(-0.5))
  gen:set_particle_size_dist(gen:fixed_dist(1.0), gen:fixed_dist(1.0))
  effect:add_anim(gen)
  effect:apply()
end

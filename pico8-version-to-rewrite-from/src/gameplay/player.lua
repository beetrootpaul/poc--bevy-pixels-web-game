-- -- -- -- -- -- -- --
-- gameplay/player   --
-- -- -- -- -- -- -- --

function new_player()
    -- TODO
    --local x = a.game_area_w / 2
    --local y = a.game_area_h / 2
    --local r = 3

    -- TODO
    --local speed = 2
    --local dx = speed
    --local dy = 0

    -- TODO
    --local direction = "r"
    --local sprite_for_direction = {
    --    u = 39,
    --    r = 40,
    --    d = 41,
    --    l = 42,
    --}

    local p = {}

    --

    -- TODO
    --function p.x1()
    --    return x - r
    --end
    --function p.xc()
    --    return x
    --end
    --function p.x2()
    --    return x + r
    --end
    --function p.y1()
    --    return y - r
    --end
    --function p.yc()
    --    return y
    --end
    --function p.y2()
    --    return y + r
    --end
    --function p.r()
    --    return r
    --end
    --function p.direction()
    --    return direction
    --end

    --

    function p.collision_circle()
        -- TODO
        --return { x = x, y = y, r = r }
    end

    --

    -- TODO
    --function p.direct_left()
    --    dx, dy = -speed, 0
    --    direction = "l"
    --end
    --function p.direct_right()
    --    dx, dy = speed, 0
    --    direction = "r"
    --end
    --function p.direct_up()
    --    dx, dy = 0, -speed
    --    direction = "u"
    --end
    --function p.direct_down()
    --    dx, dy = 0, speed
    --    direction = "d"
    --end

    --

    -- TODO
    --function p.move()
    --    x = x + dx
    --    y = y + dy
    --    x = mid(r, x, a.game_area_w - r - 1)
    --    y = mid(r, y, a.game_area_h - r - 1)
    --end

    --

    -- TODO
    --function p.draw()
    --    palt(u.colors.black, false)
    --    palt(u.colors.dark_blue, true)
    --    spr(
    --            sprite_for_direction[direction],
    --            x - r,
    --            y - r
    --    )
    --    palt()
    --    if __debug__ then
    --        circfill(x, y, r, u.colors.red)
    --    end
    --end

    --

    return p
end
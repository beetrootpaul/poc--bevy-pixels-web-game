-- -- -- -- -- -- --
-- global/utils   --
-- -- -- -- -- -- --

u = {
    buttons = {
        -- left, right, up, down
        l = 0,
        r = 1,
        u = 2,
        d = 3,
        -- TODO
        -- button O (Z), button X
        --o = 4,
        --x = 5,
    },

    -- TODO
    --colors = {
    --    black = 0,
    --    dark_blue = 1,
    --    purple = 2,
    --    dark_green = 3,
    --    brown = 4,
    --    dark_grey = 5,
    --    light_grey = 6,
    --    white = 7,
    --    red = 8,
    --    orange = 9,
    --    yellow = 10,
    --    lime = 11,
    --    blue = 12,
    --    violet_grey = 13,
    --    pink = 14,
    --    salmon = 15,
    --},

    screen_px = 128,
    -- TODO
    --screen_tiles = 16,

    -- TODO
    --text_height_px = 5,

    -- TODO
    --tile_px = 8,
}

-- TODO
--function u.boolean_changing_every_nth_second(n)
--    return ceil(sin(time() * 0.5 / n) / 2) == 1
--end

-- TODO
--function u.measure_text_width(text)
--    local y_to_print_outside_screen = a.camera_y - u.text_height_px
--    return print(text, 0, y_to_print_outside_screen) - 1
--end

-- TODO
--function u.print_with_outline(text, x, y, text_color, outline_color)
-- Docs on Control Codes: https://www.lexaloffle.com/dl/docs/pico-8_manual.html#Control_Codes
--for control_code in all(split "\-f,\-h,\|f,\|h,\+ff,\+hh,\+fh,\+hf") do
--    print(control_code .. text, x, y, outline_color)
--end
--print(text, x, y, text_color)
--end

-- TODO
--function u.trim(text)
--    local result = text
--    while sub(result, 1, 1) == ' ' do
--        result = sub(result, 2)
--    end
--    while sub(result, #result, #result) == ' ' do
--        result = sub(result, 0, #result - 1)
--    end
--    return result
--end

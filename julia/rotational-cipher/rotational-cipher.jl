function rotate(n, c::AbstractChar)
    if 'A' <= c <= 'Z'
        first = 'A'
    elseif 'a' <= c <= 'z'
        first = 'a'
    else
        return c
    end

    (c + n - first) % 26 + first
end

function rotate(n, str::String)
    String(map(c -> rotate(n, c), collect(str)))
end

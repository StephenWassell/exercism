function rotate(n, chr::AbstractChar)
    if 'A' <= chr <= 'Z'
        first = 'A'
    elseif 'a' <= chr <= 'z'
        first = 'a'
    else
        return chr
    end

    (chr + n - first) % 26 + first
end

function rotate(n, str::String)
    String(map(chr -> rotate(n, chr), collect(str)))
end

# Create non standard string literals R1 .. R25
for i = 1:25
    name = Symbol("R", i, "_str")
    @eval begin
        macro $name(str)
            rotate($i, str)
        end
    end
end

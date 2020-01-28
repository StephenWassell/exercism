const alphabet = collect('a':'z')

function letters(str::AbstractString)
	unique(sort(filter(isletter, collect(lowercase(str)))))
end

function ispangram(input::AbstractString)
	letters(input) == alphabet
end

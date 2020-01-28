function count_nucleotides(strand::AbstractString)
    counts = Dict(('A', 'C', 'G', 'T') .=> 0)
    
    for c in strand
        try
            counts[c] += 1
        catch
            throw(DomainError(c, "invalid nucleotide : $c"))
        end
    end
    
    counts
end

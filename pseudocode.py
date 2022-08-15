def viterbi_search_for_IEEs(i, search_distance):
    pass

def generate_all_permutations_of_CRC(m):
    pass

def discard_catastrophic_IEEs(IEEs):
    pass

def number_of_codewords_divisible_by_CRC_at_distance(codewords):
    pass

def current_candidates(min_divisible_at_distance, d, CRC):
    pass

def collect_IEEs(ZT_Trellis, search_distance, v):
    # v is the number of memory registers + 1
    # initialize empty lists for each state
    IEEs = [ [] * ( 2^v - 1 ) ] 
    for i in range(0, 2^v - 1):
        IEEs[i] = (viterbi_search_for_IEEs(i, search_distance))
    return IEEs


def search_for_best_CRC(Trellis_len, m, IEEs, search_distance):
    candidate_crcs = generate_all_permutations_of_CRC(m) # full list we use for reference
    non_catastrophic_IEEs = discard_catastrophic_IEEs(IEEs) # this is hugely comp. intensive
    candidates = [[] * search_distance] # mutable list that we update as we whittle down our options
    candidates[0] = candidate_crcs
    for d in range(0, search_distance):
        count = []
        for CRC, i in enumerate(candidate_crcs):
            codewords = non_catastrophic_IEEs[i]
            count[i] = number_of_codewords_divisible_by_CRC_at_distance(codewords, CRC)
        min_divisible_at_distance = min(count)
        candidates[d + 1] = current_candidates(candidate_crcs, min_divisible_at_distance, d, CRC)
        if candidates[d + 1].len() == 1:
            return candidates[d + 1]

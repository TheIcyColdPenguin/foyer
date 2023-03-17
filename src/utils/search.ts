export const search = (searchTerm: string, corpus: string[]) => {
    if (!searchTerm) {
        return -1;
    }

    const results = [];

    outer: for (let i = 0; i < corpus.length; i++) {
        const corpusItem = corpus[i];
        let currIndex = corpusItem.indexOf(searchTerm[0]);

        if (currIndex === -1) {
            continue;
        }

        for (let i = 1; i < searchTerm.length; i++) {
            let nextIndex = corpusItem.slice(currIndex + 1).indexOf(searchTerm[i]);
            if (nextIndex === -1) {
                continue outer;
            } else {
                currIndex += nextIndex + 1;
            }
        }

        return i;
    }

    return -1;
};

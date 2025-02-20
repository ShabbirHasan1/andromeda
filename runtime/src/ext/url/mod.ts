// deno-lint-ignore-file no-unused-vars
class URL {
    url: string;
    base?: string;
    serialized: string;
    constructor(url: string, base?: string) {
        this.url = url;
        this.base = base;
        this.serialized = base
            ? internal_url_parse(url, base)
            : internal_url_parse_no_base(url);
    }

    toString() {
        return this.serialized;
    }
}

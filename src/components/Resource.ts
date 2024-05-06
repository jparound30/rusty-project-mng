export class Resource {
    resource_id: number = 0;
    name: string = "";
    cost_per_month: number = 0;

    public reset() {
        this.resource_id = 0
        this.name = ""
        this.cost_per_month = 0
    }
}


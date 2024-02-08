export class TaskFull {
    task_id: number = 0;
    title: string = "";
    description: string | null = null;
    assignee_resource_id: number | null = null;
    assignee_resource_name: string | null = null;
    parent_task_id: number | null = null;
    parent_task_title: string | null = null;
    start_date: string | null = null;
    due_date: string | null = null;
    estimated_time: number | null = null;
    actual_time: number | null = null;
    planned_value: number | null = null;
    task_status_id: number = 0;
    task_status_name: String = "";
    progress_rate: number = 0;
}

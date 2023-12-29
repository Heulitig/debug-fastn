export {};

declare global {
    interface Window {
        ftd: any;
        enable_dark_mode(): void;
        enable_light_mode(): void;
        enable_system_mode(): void;
        [key: string]: any;
    }

    interface Export {
        init: object;
        data: object;
        handle_event(evt: Event, id: string, event: string, obj: Element): void;
        handle_function(evt: Event, id: string, event: string, obj: Element): any;
        set_string_for_all(variable: string, value: string): any;
        set_bool_for_all(variable: string, value: boolean): any;
        set_bool(id: string, variable: string, value: boolean): any;
        set_value(variable: string, value: any): any;
        set_value_by_id(id: string, variable: string, value: any): any;
        get_value(id: string, variable: string): any;
        is_empty(str: any): boolean;
        set_list(array: any[], value: any[], args: any, data: any, id: string): any[];
        append(array: any[], value: any, args: any, data: any, id: string): any[];
        clear(array: any[], args: any, data: any, id: string): any[];
        insert_at(array: any[], value: any, idx: number, args: any, data: any, id: string): any[];
        delete_at(array: any[], idx: number, args: any, data: any, id: string): any[];
        copy_to_clipboard(text: string): void;
        set_rive_boolean(canva_id: string, input: string, value: boolean, args: any, data: any, id: string): void;
        toggle_rive_boolean(canva_id: string, input: string, args: any, data: any, id: string): void;
        set_rive_integer(canva_id: string, input: string, value: bigint, args: any, data: any, id: string): void;
        fire_rive(canva_id: string, input: string, args: any, data: any, id: string): void;
        play_rive(canva_id: string, input: string, args: any, data: any, id: string): void;
        pause_rive(canva_id: string, input: string, args: any, data: any, id: string): void;
        toggle_play_rive(canva_id: string, input: string, args: any, data: any, id: string): void;
        http(url: string, method: string, ...request_data: any): void;
        component_data(component: HTMLElement): any;
        create_list(array_name: string, id: string): void;
        delete_list(array_name: string, id: string): void;
        call_mutable_value_changes(key: string, id: string): void;
        call_immutable_value_changes(key: string, id: string): void;
    }

    interface String {
        format(...args: any[]): String;
        replace_format(...args: any[]): String;
    }
}

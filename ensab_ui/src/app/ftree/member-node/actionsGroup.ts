import { WritableSignal, signal } from "@angular/core";

export default class ActionsGroup {
    take_action : WritableSignal<boolean>
    add_son : WritableSignal<boolean>
    remove_son : WritableSignal<boolean>
    rename_son : WritableSignal<boolean>

    constructor() {
        this.take_action = signal(false)
        this.add_son = signal(false)
        this.remove_son = signal(false)
        this.rename_son = signal(false)
    }

    toggle_action() {
        this.take_action.update(x => !x);  
    } 

    add_son_action() {
        this.add_son.set(true)
        this.toggle_action()
    }

    add_son_done() {
        this.add_son.set(false)
    }

    remove_son_action() {
        this.remove_son.set(true)
        this.toggle_action()
    }

    remove_son_done() {
        this.remove_son.set(false)
    }

    rename_son_action(){
        this.rename_son.set(true)
        this.toggle_action()
    }

    rename_son_done(){
        this.rename_son.set(false)
    }
}
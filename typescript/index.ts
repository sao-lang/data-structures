const sleep = (delay: number) => {
    return new Promise((resolve: (value: boolean) => void) => {
        setTimeout(() => {
            resolve(true);
        }, delay * 1000);
    })
}

const lazyExecute = (fn: () => Promise<any>, delay: number) => {
    return new Promise(async (resolve: any, reject) => {
        setTimeout(() => {
            reject();
        }, delay * 1000);
        const res = await fn();
        resolve(res);
    })
}

const flatten = (arr: any[]) => {
    const result: any[] = [];
    for (const element of arr) {
        result.push(Array.isArray(element) ? flatten(arr) : element);
    }
    return result;
}

const debounce = (fn: () => void, delay: number) => {
    let timer: any = null;
    return function (this: any, ...args: any[]) {
        if (timer) {
            clearTimeout(timer);
        }
        timer = setTimeout(() => {
            fn.apply(this, args as any);
        }, delay);
    }
}

class EventBus {
    private tasks: ({ key: string, cbs: (() => void)[] })[] = [];

    on(key: string, callback: () => void) {
        const target = this.tasks.find(ele => ele.key === key)
        if (target) {
            target.cbs.push(callback);
            return;
        }
        this.tasks.push({ key, cbs: [callback] });
    }
    off(key: string, callback: () => void) {
        const target = this.tasks.find(ele => ele.key === key)
        if (target) {
            target.cbs.push(callback);
            return;
        }
    }
    emit (key: string) {
        this.tasks.forEach((ele) => {
            ele.cbs.forEach((cb) => {
                cb();
            })
        })
    }
}

const middle = () => {
    // 声明一些变量
    return (ctx: any) => {
        // 
        ctx.Next();
        
    }
}
type AnyObject = Record<string, unknown>;

// --- 1. 字符串转换辅助函数 ---

/**
 * 将驼峰命名的字符串转换为下划线命名。
 */
const camelToSnake = (str: string): string => str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);

/**
 * 将下划线命名的字符串转换为驼峰命名。
 */
const snakeToCamel = (str: string): string => str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());

// --- 2. 核心递归转换函数 ---

/**
 * 递归地转换对象或数组中的键。
 * @param data - 要转换的数据（对象、数组或原始值）。
 * @param keyConverter - 用于转换每个键的函数（例如 camelToSnake）。
 * @returns 转换后的新数据。
 */
function deepConvertKeys(data: any, keyConverter: (key: string) => string): any {
  // 如果是数组，则递归处理数组中的每一项
  if (Array.isArray(data)) {
    return data.map(item => deepConvertKeys(item, keyConverter));
  }

  // 如果是对象（但不是数组或 null），则处理其键
  if (data && typeof data === "object" && !Array.isArray(data)) {
    // 使用 Object.entries 和 Object.fromEntries 来优雅地处理对象
    return Object.fromEntries(
      Object.entries(data).map(([key, value]) => [
        // 转换键
        keyConverter(key),
        // 递归转换值
        deepConvertKeys(value, keyConverter),
      ])
    );
  }

  // 如果是原始类型，则直接返回
  return data;
}

/**
 * 将一个对象（包括其嵌套内容）的所有键从驼峰命名转换为下划线命名。
 * @param obj - 要转换的对象。
 * @returns 一个新的对象，其所有键都已转换为下划线命名。
 */
export function keysToSnakeCase<T extends AnyObject>(obj: T): AnyObject {
  return deepConvertKeys(obj, camelToSnake);
}

/**
 * 将一个对象（包括其嵌套内容）的所有键从下划线命名转换为驼峰命名。
 * @param obj - 要转换的对象。
 * @returns 一个新的对象，其所有键都已转换为驼峰命名。
 */
export function keysToCamelCase<T extends AnyObject>(obj: T): AnyObject {
  return deepConvertKeys(obj, snakeToCamel);
}

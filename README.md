# Example

```diff
-fallible().unwrap()
+(|| fallible()).unwrap_retry().await
```
or
```diff
-fallible().unwrap()
+(|| fallible()).unwrap_blocking()
```

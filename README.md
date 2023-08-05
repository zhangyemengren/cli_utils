# cli-utils

# 说明
## dev
```bash
 cargo run -- update --dir /Users/zhangchi/projects --branch qwer4 --package react --version 11
```
# todo
- [x] 传入参数，指定文件夹，指定分支
- [x] 无分支时，自动创建分支，提交处理不同  git push --set-upstream origin branch-name
- [x] 多线程 传入work space
- [ ] 完善终止判断/错误信息
- [ ] 排除 当排除存在时 从work space中排除
- [ ] 选择 当选择存在 优先级最高 只处理选择的文件 忽略排除
- [x] 传入包name及版本

# feature
- [ ] 更新包依赖
- [ ] 检查git状态 未提交 未推送 working tree clean
- [ ] 自动合并master
- [ ] 文案导入
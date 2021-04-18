
../target/riscv64imac-unknown-none-elf/release//user_task:     file format elf64-littleriscv


Disassembly of section .text:

0000000000000000 <_start>:
       0:	1101                	addi	sp,sp,-32
       2:	ec06                	sd	ra,24(sp)
       4:	e822                	sd	s0,16(sp)
       6:	e426                	sd	s1,8(sp)

0000000000000008 <.LBB2_3>:
       8:	0000f517          	auipc	a0,0xf
       c:	03050513          	addi	a0,a0,48 # f038 <_ZN12tornado_user4HEAP17h1339c640c584f725E>
      10:	00002097          	auipc	ra,0x2
      14:	daa080e7          	jalr	-598(ra) # 1dba <_ZN78_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..ops..deref..Deref$GT$5deref17h9a8f4477244c971aE>
      18:	842a                	mv	s0,a0
      1a:	4505                	li	a0,1
      1c:	00a434af          	amoadd.d	s1,a0,(s0)
      20:	6408                	ld	a0,8(s0)
      22:	0230000f          	fence	r,rw
      26:	fe951de3          	bne	a0,s1,20 <.LBB2_3+0x18>
      2a:	01040513          	addi	a0,s0,16

000000000000002e <.LBB2_4>:
      2e:	00007597          	auipc	a1,0x7
      32:	00a58593          	addi	a1,a1,10 # 7038 <_ZN12tornado_user10HEAP_SPACE17hcda5f67e49da29e7E>
      36:	6621                	lui	a2,0x8
      38:	00001097          	auipc	ra,0x1
      3c:	74c080e7          	jalr	1868(ra) # 1784 <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E>
      40:	00148513          	addi	a0,s1,1
      44:	0310000f          	fence	rw,w
      48:	e408                	sd	a0,8(s0)
      4a:	850e                	mv	a0,gp

000000000000004c <.LBB2_5>:
      4c:	0000f597          	auipc	a1,0xf
      50:	12458593          	addi	a1,a1,292 # f170 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE>
      54:	e188                	sd	a0,0(a1)
      56:	8512                	mv	a0,tp

0000000000000058 <.LBB2_6>:
      58:	0000f597          	auipc	a1,0xf
      5c:	11058593          	addi	a1,a1,272 # f168 <_ZN12tornado_user16ADDRESS_SPACE_ID17hd7ca12c82c04119fE>
      60:	e188                	sd	a0,0(a1)
      62:	00000097          	auipc	ra,0x0
      66:	06a080e7          	jalr	106(ra) # cc <main>
	...

000000000000006c <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351>:
      6c:	1101                	addi	sp,sp,-32
      6e:	ec06                	sd	ra,24(sp)
      70:	e822                	sd	s0,16(sp)
      72:	e426                	sd	s1,8(sp)
      74:	e04a                	sd	s2,0(sp)
      76:	84ae                	mv	s1,a1
      78:	842a                	mv	s0,a0
      7a:	ce11                	beqz	a2,96 <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x2a>
      7c:	8932                	mv	s2,a2
      7e:	6288                	ld	a0,0(a3)
      80:	cd19                	beqz	a0,9e <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x32>
      82:	668c                	ld	a1,8(a3)
      84:	cd89                	beqz	a1,9e <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x32>
      86:	864a                	mv	a2,s2
      88:	86a6                	mv	a3,s1
      8a:	00001097          	auipc	ra,0x1
      8e:	a88080e7          	jalr	-1400(ra) # b12 <__rust_realloc>
      92:	e11d                	bnez	a0,b8 <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x4c>
      94:	a829                	j	ae <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x42>
      96:	e404                	sd	s1,8(s0)
      98:	4585                	li	a1,1
      9a:	4481                	li	s1,0
      9c:	a005                	j	bc <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x50>
      9e:	cc81                	beqz	s1,b6 <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x4a>
      a0:	8526                	mv	a0,s1
      a2:	85ca                	mv	a1,s2
      a4:	00001097          	auipc	ra,0x1
      a8:	a5e080e7          	jalr	-1442(ra) # b02 <__rust_alloc>
      ac:	e511                	bnez	a0,b8 <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x4c>
      ae:	e404                	sd	s1,8(s0)
      b0:	4585                	li	a1,1
      b2:	84ca                	mv	s1,s2
      b4:	a021                	j	bc <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x50>
      b6:	854a                	mv	a0,s2
      b8:	4581                	li	a1,0
      ba:	e408                	sd	a0,8(s0)
      bc:	e804                	sd	s1,16(s0)
      be:	e00c                	sd	a1,0(s0)
      c0:	6902                	ld	s2,0(sp)
      c2:	64a2                	ld	s1,8(sp)
      c4:	6442                	ld	s0,16(sp)
      c6:	60e2                	ld	ra,24(sp)
      c8:	6105                	addi	sp,sp,32
      ca:	8082                	ret

00000000000000cc <main>:
      cc:	7131                	addi	sp,sp,-192
      ce:	fd06                	sd	ra,184(sp)
      d0:	f922                	sd	s0,176(sp)
      d2:	f526                	sd	s1,168(sp)
      d4:	f14a                	sd	s2,160(sp)
      d6:	ed4e                	sd	s3,152(sp)
      d8:	e952                	sd	s4,144(sp)
      da:	e556                	sd	s5,136(sp)
      dc:	4551                	li	a0,20
      de:	4591                	li	a1,4
      e0:	4491                	li	s1,4
      e2:	00001097          	auipc	ra,0x1
      e6:	a20080e7          	jalr	-1504(ra) # b02 <__rust_alloc>
      ea:	c90d                	beqz	a0,11c <main+0x50>
      ec:	842a                	mv	s0,a0
      ee:	e02a                	sd	a0,0(sp)
      f0:	4915                	li	s2,5
      f2:	e44a                	sd	s2,8(sp)
      f4:	e84a                	sd	s2,16(sp)
      f6:	4989                	li	s3,2
      f8:	01352023          	sw	s3,0(a0)
      fc:	4a0d                	li	s4,3
      fe:	01452223          	sw	s4,4(a0)
     102:	c504                	sw	s1,8(a0)
     104:	01252623          	sw	s2,12(a0)
     108:	4a99                	li	s5,6
     10a:	01552823          	sw	s5,16(a0)
     10e:	4551                	li	a0,20
     110:	4591                	li	a1,4
     112:	00001097          	auipc	ra,0x1
     116:	9f0080e7          	jalr	-1552(ra) # b02 <__rust_alloc>
     11a:	e901                	bnez	a0,12a <main+0x5e>
     11c:	4551                	li	a0,20
     11e:	4591                	li	a1,4
     120:	00002097          	auipc	ra,0x2
     124:	d26080e7          	jalr	-730(ra) # 1e46 <_ZN5alloc5alloc18handle_alloc_error17h0809b9ba7eebe66bE>
     128:	0000                	unimp
     12a:	84aa                	mv	s1,a0
     12c:	01352023          	sw	s3,0(a0)
     130:	01452223          	sw	s4,4(a0)
     134:	4511                	li	a0,4
     136:	c488                	sw	a0,8(s1)
     138:	0124a623          	sw	s2,12(s1)
     13c:	0154a823          	sw	s5,16(s1)
     140:	e0a6                	sd	s1,64(sp)
     142:	e4ca                	sd	s2,72(sp)
     144:	e8ca                	sd	s2,80(sp)
     146:	4651                	li	a2,20
     148:	8522                	mv	a0,s0
     14a:	85a6                	mv	a1,s1
     14c:	00004097          	auipc	ra,0x4
     150:	ed2080e7          	jalr	-302(ra) # 401e <bcmp>
     154:	e971                	bnez	a0,228 <.LBB2_12+0x16>
     156:	45d1                	li	a1,20
     158:	4611                	li	a2,4
     15a:	8526                	mv	a0,s1
     15c:	00001097          	auipc	ra,0x1
     160:	9ae080e7          	jalr	-1618(ra) # b0a <__rust_dealloc>
     164:	ec82                	sd	zero,88(sp)
     166:	4905                	li	s2,1
     168:	f0ca                	sd	s2,96(sp)
     16a:	f482                	sd	zero,104(sp)
     16c:	4999                	li	s3,6
     16e:	f8ce                	sd	s3,112(sp)
     170:	08a8                	addi	a0,sp,88
     172:	00001097          	auipc	ra,0x1
     176:	80a080e7          	jalr	-2038(ra) # 97c <_ZN12tornado_user7excutor5spawn17h7b2a2b919cdc5b7aE>
     17a:	00001097          	auipc	ra,0x1
     17e:	b22080e7          	jalr	-1246(ra) # c9c <_ZN12tornado_user7excutor8try_join17h027004220a5c13bdE>
     182:	ec2a                	sd	a0,24(sp)
     184:	00154513          	xori	a0,a0,1
     188:	0085c613          	xori	a2,a1,8
     18c:	8d51                	or	a0,a0,a2
     18e:	f02e                	sd	a1,32(sp)
     190:	e955                	bnez	a0,244 <.LBB2_13+0x1a>

0000000000000192 <.LBB2_10>:
     192:	0000f517          	auipc	a0,0xf
     196:	fde50513          	addi	a0,a0,-34 # f170 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE>
     19a:	6100                	ld	s0,0(a0)
     19c:	4501                	li	a0,0
     19e:	9402                	jalr	s0
     1a0:	8a2a                	mv	s4,a0
     1a2:	4505                	li	a0,1
     1a4:	9402                	jalr	s0
     1a6:	84aa                	mv	s1,a0
     1a8:	4509                	li	a0,2
     1aa:	9402                	jalr	s0
     1ac:	f426                	sd	s1,40(sp)
     1ae:	f82a                	sd	a0,48(sp)
     1b0:	9a02                	jalr	s4
     1b2:	842a                	mv	s0,a0
     1b4:	fc2a                	sd	a0,56(sp)
     1b6:	ec82                	sd	zero,88(sp)
     1b8:	f0ca                	sd	s2,96(sp)
     1ba:	f482                	sd	zero,104(sp)
     1bc:	f8ce                	sd	s3,112(sp)
     1be:	08a8                	addi	a0,sp,88
     1c0:	00000097          	auipc	ra,0x0
     1c4:	16c080e7          	jalr	364(ra) # 32c <_ZN12tornado_user4task8UserTask3new17h142089b4a4db545fE>
     1c8:	85aa                	mv	a1,a0
     1ca:	04010913          	addi	s2,sp,64
     1ce:	854a                	mv	a0,s2
     1d0:	00001097          	auipc	ra,0x1
     1d4:	ef0080e7          	jalr	-272(ra) # 10c0 <_ZN12tornado_user4task8UserTask18shared_task_handle17h946cb8771506c97bE>
     1d8:	08a8                	addi	a0,sp,88
     1da:	85a2                	mv	a1,s0
     1dc:	864a                	mv	a2,s2
     1de:	9482                	jalr	s1
     1e0:	1808                	addi	a0,sp,48
     1e2:	1030                	addi	a2,sp,40
     1e4:	182c                	addi	a1,sp,56
     1e6:	86ae                	mv	a3,a1
     1e8:	00000097          	auipc	ra,0x0
     1ec:	30a080e7          	jalr	778(ra) # 4f2 <_ZN12tornado_user6shared15run_until_ready17he93e42bfed9e9094E>
     1f0:	e0aa                	sd	a0,64(sp)
     1f2:	00154513          	xori	a0,a0,1
     1f6:	0085c613          	xori	a2,a1,8
     1fa:	8d51                	or	a0,a0,a2
     1fc:	e4ae                	sd	a1,72(sp)
     1fe:	e525                	bnez	a0,266 <.LBB2_15+0x18>
     200:	4501                	li	a0,0
     202:	00001097          	auipc	ra,0x1
     206:	37a080e7          	jalr	890(ra) # 157c <_ZN12tornado_user4exit17h19453ae3096d9226E>

000000000000020a <.LBB2_11>:
     20a:	00005517          	auipc	a0,0x5
     20e:	e9650513          	addi	a0,a0,-362 # 50a0 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.5>

0000000000000212 <.LBB2_12>:
     212:	00005617          	auipc	a2,0x5
     216:	eb660613          	addi	a2,a2,-330 # 50c8 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.6>
     21a:	02800593          	li	a1,40
     21e:	00002097          	auipc	ra,0x2
     222:	d24080e7          	jalr	-732(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
     226:	0000                	unimp
     228:	ec82                	sd	zero,88(sp)

000000000000022a <.LBB2_13>:
     22a:	00005717          	auipc	a4,0x5
     22e:	dfe70713          	addi	a4,a4,-514 # 5028 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.1>
     232:	858a                	mv	a1,sp
     234:	0090                	addi	a2,sp,64
     236:	08b4                	addi	a3,sp,88
     238:	4501                	li	a0,0
     23a:	00000097          	auipc	ra,0x0
     23e:	708080e7          	jalr	1800(ra) # 942 <_ZN4core9panicking13assert_failed17hb943cda24c27692cE>
     242:	0000                	unimp
     244:	ec82                	sd	zero,88(sp)

0000000000000246 <.LBB2_14>:
     246:	00005617          	auipc	a2,0x5
     24a:	e0a60613          	addi	a2,a2,-502 # 5050 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.1+0x28>

000000000000024e <.LBB2_15>:
     24e:	00005717          	auipc	a4,0x5
     252:	e2270713          	addi	a4,a4,-478 # 5070 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.3>
     256:	082c                	addi	a1,sp,24
     258:	08b4                	addi	a3,sp,88
     25a:	4501                	li	a0,0
     25c:	00000097          	auipc	ra,0x0
     260:	6ac080e7          	jalr	1708(ra) # 908 <_ZN4core9panicking13assert_failed17h89f31df7e7d23afbE>
     264:	0000                	unimp
     266:	ec82                	sd	zero,88(sp)

0000000000000268 <.LBB2_16>:
     268:	00005617          	auipc	a2,0x5
     26c:	de860613          	addi	a2,a2,-536 # 5050 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.1+0x28>

0000000000000270 <.LBB2_17>:
     270:	00005717          	auipc	a4,0x5
     274:	e1870713          	addi	a4,a4,-488 # 5088 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.4>
     278:	008c                	addi	a1,sp,64
     27a:	08b4                	addi	a3,sp,88
     27c:	4501                	li	a0,0
     27e:	00000097          	auipc	ra,0x0
     282:	68a080e7          	jalr	1674(ra) # 908 <_ZN4core9panicking13assert_failed17h89f31df7e7d23afbE>
	...

0000000000000288 <_ZN75_$LT$user_task..FibonacciFuture$u20$as$u20$core..future..future..Future$GT$4poll17he49c34713d0f221dE>:
     288:	1141                	addi	sp,sp,-16
     28a:	e406                	sd	ra,8(sp)
     28c:	e022                	sd	s0,0(sp)
     28e:	6910                	ld	a2,16(a0)
     290:	6d14                	ld	a3,24(a0)
     292:	6100                	ld	s0,0(a0)
     294:	00d61463          	bne	a2,a3,29c <_ZN75_$LT$user_task..FibonacciFuture$u20$as$u20$core..future..future..Future$GT$4poll17he49c34713d0f221dE+0x14>
     298:	4501                	li	a0,0
     29a:	a829                	j	2b4 <_ZN75_$LT$user_task..FibonacciFuture$u20$as$u20$core..future..future..Future$GT$4poll17he49c34713d0f221dE+0x2c>
     29c:	6514                	ld	a3,8(a0)
     29e:	96a2                	add	a3,a3,s0
     2a0:	e114                	sd	a3,0(a0)
     2a2:	e500                	sd	s0,8(a0)
     2a4:	0605                	addi	a2,a2,1
     2a6:	e910                	sd	a2,16(a0)
     2a8:	6188                	ld	a0,0(a1)
     2aa:	650c                	ld	a1,8(a0)
     2ac:	698c                	ld	a1,16(a1)
     2ae:	6108                	ld	a0,0(a0)
     2b0:	9582                	jalr	a1
     2b2:	4505                	li	a0,1
     2b4:	85a2                	mv	a1,s0
     2b6:	6402                	ld	s0,0(sp)
     2b8:	60a2                	ld	ra,8(sp)
     2ba:	0141                	addi	sp,sp,16
     2bc:	8082                	ret

00000000000002be <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0042b06b0b945dcfE>:
     2be:	715d                	addi	sp,sp,-80
     2c0:	e486                	sd	ra,72(sp)
     2c2:	e0a2                	sd	s0,64(sp)
     2c4:	fc26                	sd	s1,56(sp)
     2c6:	f84a                	sd	s2,48(sp)
     2c8:	f44e                	sd	s3,40(sp)
     2ca:	f052                	sd	s4,32(sp)
     2cc:	ec56                	sd	s5,24(sp)
     2ce:	6108                	ld	a0,0(a0)
     2d0:	6104                	ld	s1,0(a0)
     2d2:	6900                	ld	s0,16(a0)
     2d4:	852e                	mv	a0,a1
     2d6:	00003097          	auipc	ra,0x3
     2da:	b4c080e7          	jalr	-1204(ra) # 2e22 <_ZN4core3fmt9Formatter10debug_list17h14691806d5f8258eE>
     2de:	e42e                	sd	a1,8(sp)
     2e0:	e02a                	sd	a0,0(sp)
     2e2:	c415                	beqz	s0,30e <.LBB0_4+0x28>
     2e4:	040a                	slli	s0,s0,0x2

00000000000002e6 <.LBB0_4>:
     2e6:	00005a17          	auipc	s4,0x5
     2ea:	dfaa0a13          	addi	s4,s4,-518 # 50e0 <anon.8e5db402a9e16f5ceff60bed8bcd7dec.0.llvm.14341049042176442005>
     2ee:	890a                	mv	s2,sp
     2f0:	01010993          	addi	s3,sp,16
     2f4:	00448a93          	addi	s5,s1,4
     2f8:	e826                	sd	s1,16(sp)
     2fa:	854a                	mv	a0,s2
     2fc:	85ce                	mv	a1,s3
     2fe:	8652                	mv	a2,s4
     300:	00002097          	auipc	ra,0x2
     304:	13a080e7          	jalr	314(ra) # 243a <_ZN4core3fmt8builders8DebugSet5entry17hfe13052758799ed9E>
     308:	1471                	addi	s0,s0,-4
     30a:	84d6                	mv	s1,s5
     30c:	f465                	bnez	s0,2f4 <.LBB0_4+0xe>
     30e:	850a                	mv	a0,sp
     310:	00002097          	auipc	ra,0x2
     314:	144080e7          	jalr	324(ra) # 2454 <_ZN4core3fmt8builders9DebugList6finish17h92fc23c61d612d60E>
     318:	6ae2                	ld	s5,24(sp)
     31a:	7a02                	ld	s4,32(sp)
     31c:	79a2                	ld	s3,40(sp)
     31e:	7942                	ld	s2,48(sp)
     320:	74e2                	ld	s1,56(sp)
     322:	6406                	ld	s0,64(sp)
     324:	60a6                	ld	ra,72(sp)
     326:	6161                	addi	sp,sp,80
     328:	8082                	ret

000000000000032a <_ZN4core3ptr28drop_in_place$LT$$RF$i32$GT$17h2eb012521c49cc55E.llvm.14341049042176442005>:
     32a:	8082                	ret

000000000000032c <_ZN12tornado_user4task8UserTask3new17h142089b4a4db545fE>:
     32c:	715d                	addi	sp,sp,-80
     32e:	e486                	sd	ra,72(sp)
     330:	e0a2                	sd	s0,64(sp)
     332:	fc26                	sd	s1,56(sp)
     334:	f84a                	sd	s2,48(sp)
     336:	842a                	mv	s0,a0
     338:	00001097          	auipc	ra,0x1
     33c:	d56080e7          	jalr	-682(ra) # 108e <_ZN12tornado_user4task10UserTaskId8generate17hb5138570cf78f129E>

0000000000000340 <.LBB0_6>:
     340:	0000f597          	auipc	a1,0xf
     344:	e2858593          	addi	a1,a1,-472 # f168 <_ZN12tornado_user16ADDRESS_SPACE_ID17hd7ca12c82c04119fE>
     348:	618c                	ld	a1,0(a1)
     34a:	84aa                	mv	s1,a0
     34c:	852e                	mv	a0,a1
     34e:	00001097          	auipc	ra,0x1
     352:	1e8080e7          	jalr	488(ra) # 1536 <_ZN12tornado_user6shared14AddressSpaceId8from_raw17hbb7124f5610e8528E>
     356:	892a                	mv	s2,a0
     358:	6c08                	ld	a0,24(s0)
     35a:	680c                	ld	a1,16(s0)
     35c:	6410                	ld	a2,8(s0)
     35e:	6014                	ld	a3,0(s0)
     360:	f42a                	sd	a0,40(sp)
     362:	f02e                	sd	a1,32(sp)
     364:	ec32                	sd	a2,24(sp)
     366:	e836                	sd	a3,16(sp)
     368:	02000513          	li	a0,32
     36c:	45a1                	li	a1,8
     36e:	00000097          	auipc	ra,0x0
     372:	794080e7          	jalr	1940(ra) # b02 <__rust_alloc>
     376:	c155                	beqz	a0,41a <.LBB0_7+0x3a>
     378:	842a                	mv	s0,a0
     37a:	7522                	ld	a0,40(sp)
     37c:	7582                	ld	a1,32(sp)
     37e:	6662                	ld	a2,24(sp)
     380:	66c2                	ld	a3,16(sp)
     382:	ec08                	sd	a0,24(s0)
     384:	e80c                	sd	a1,16(s0)
     386:	e410                	sd	a2,8(s0)
     388:	e014                	sd	a3,0(s0)
     38a:	03800513          	li	a0,56
     38e:	45a1                	li	a1,8
     390:	00000097          	auipc	ra,0x0
     394:	772080e7          	jalr	1906(ra) # b02 <__rust_alloc>
     398:	c541                	beqz	a0,420 <.LBB0_7+0x40>
     39a:	4585                	li	a1,1
     39c:	e10c                	sd	a1,0(a0)
     39e:	e50c                	sd	a1,8(a0)
     3a0:	01010583          	lb	a1,16(sp)
     3a4:	e904                	sd	s1,16(a0)
     3a6:	01110603          	lb	a2,17(sp)
     3aa:	01210683          	lb	a3,18(sp)
     3ae:	00b50ca3          	sb	a1,25(a0)
     3b2:	01310583          	lb	a1,19(sp)
     3b6:	00c50d23          	sb	a2,26(a0)
     3ba:	00d50da3          	sb	a3,27(a0)
     3be:	01410603          	lb	a2,20(sp)
     3c2:	00b50e23          	sb	a1,28(a0)
     3c6:	01510583          	lb	a1,21(sp)
     3ca:	01610683          	lb	a3,22(sp)
     3ce:	00c50ea3          	sb	a2,29(a0)
     3d2:	00050c23          	sb	zero,24(a0)
     3d6:	00b50f23          	sb	a1,30(a0)
     3da:	00d50fa3          	sb	a3,31(a0)
     3de:	f100                	sd	s0,32(a0)

00000000000003e0 <.LBB0_7>:
     3e0:	00005597          	auipc	a1,0x5
     3e4:	d2058593          	addi	a1,a1,-736 # 5100 <anon.ac1ba77cd7d7837b7067d1be6c826dee.0.llvm.1899440800677633648>
     3e8:	f50c                	sd	a1,40(a0)
     3ea:	03251823          	sh	s2,48(a0)
     3ee:	02050a23          	sb	zero,52(a0)
     3f2:	00d10583          	lb	a1,13(sp)
     3f6:	00e10603          	lb	a2,14(sp)
     3fa:	00f10683          	lb	a3,15(sp)
     3fe:	02051923          	sh	zero,50(a0)
     402:	02b50aa3          	sb	a1,53(a0)
     406:	02c50b23          	sb	a2,54(a0)
     40a:	02d50ba3          	sb	a3,55(a0)
     40e:	7942                	ld	s2,48(sp)
     410:	74e2                	ld	s1,56(sp)
     412:	6406                	ld	s0,64(sp)
     414:	60a6                	ld	ra,72(sp)
     416:	6161                	addi	sp,sp,80
     418:	8082                	ret
     41a:	02000513          	li	a0,32
     41e:	a019                	j	424 <.LBB0_7+0x44>
     420:	03800513          	li	a0,56
     424:	45a1                	li	a1,8
     426:	00002097          	auipc	ra,0x2
     42a:	a20080e7          	jalr	-1504(ra) # 1e46 <_ZN5alloc5alloc18handle_alloc_error17h0809b9ba7eebe66bE>
	...

0000000000000430 <_ZN4core3ptr47drop_in_place$LT$user_task..FibonacciFuture$GT$17h11c90872ba4eff3cE.llvm.1899440800677633648>:
     430:	8082                	ret

0000000000000432 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8eaf272c2eb574ceE>:
     432:	1101                	addi	sp,sp,-32
     434:	ec06                	sd	ra,24(sp)
     436:	e822                	sd	s0,16(sp)
     438:	e426                	sd	s1,8(sp)
     43a:	6104                	ld	s1,0(a0)
     43c:	842e                	mv	s0,a1
     43e:	852e                	mv	a0,a1
     440:	00003097          	auipc	ra,0x3
     444:	990080e7          	jalr	-1648(ra) # 2dd0 <_ZN4core3fmt9Formatter15debug_lower_hex17hdec886692a7b892cE>
     448:	c919                	beqz	a0,45e <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8eaf272c2eb574ceE+0x2c>
     44a:	8526                	mv	a0,s1
     44c:	85a2                	mv	a1,s0
     44e:	64a2                	ld	s1,8(sp)
     450:	6442                	ld	s0,16(sp)
     452:	60e2                	ld	ra,24(sp)
     454:	6105                	addi	sp,sp,32
     456:	00003317          	auipc	t1,0x3
     45a:	4bc30067          	jr	1212(t1) # 3912 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h68aa87d5d39609c1E>
     45e:	8522                	mv	a0,s0
     460:	00003097          	auipc	ra,0x3
     464:	97a080e7          	jalr	-1670(ra) # 2dda <_ZN4core3fmt9Formatter15debug_upper_hex17hfaedba9b5105966cE>
     468:	c919                	beqz	a0,47e <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8eaf272c2eb574ceE+0x4c>
     46a:	8526                	mv	a0,s1
     46c:	85a2                	mv	a1,s0
     46e:	64a2                	ld	s1,8(sp)
     470:	6442                	ld	s0,16(sp)
     472:	60e2                	ld	ra,24(sp)
     474:	6105                	addi	sp,sp,32
     476:	00003317          	auipc	t1,0x3
     47a:	51c30067          	jr	1308(t1) # 3992 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h898848ae100a065bE>
     47e:	8526                	mv	a0,s1
     480:	85a2                	mv	a1,s0
     482:	64a2                	ld	s1,8(sp)
     484:	6442                	ld	s0,16(sp)
     486:	60e2                	ld	ra,24(sp)
     488:	6105                	addi	sp,sp,32
     48a:	00004317          	auipc	t1,0x4
     48e:	82e30067          	jr	-2002(t1) # 3cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>

0000000000000492 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hc899b377d5fb6a1bE>:
     492:	1101                	addi	sp,sp,-32
     494:	ec06                	sd	ra,24(sp)
     496:	e822                	sd	s0,16(sp)
     498:	e426                	sd	s1,8(sp)
     49a:	6104                	ld	s1,0(a0)
     49c:	842e                	mv	s0,a1
     49e:	852e                	mv	a0,a1
     4a0:	00003097          	auipc	ra,0x3
     4a4:	930080e7          	jalr	-1744(ra) # 2dd0 <_ZN4core3fmt9Formatter15debug_lower_hex17hdec886692a7b892cE>
     4a8:	c919                	beqz	a0,4be <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hc899b377d5fb6a1bE+0x2c>
     4aa:	8526                	mv	a0,s1
     4ac:	85a2                	mv	a1,s0
     4ae:	64a2                	ld	s1,8(sp)
     4b0:	6442                	ld	s0,16(sp)
     4b2:	60e2                	ld	ra,24(sp)
     4b4:	6105                	addi	sp,sp,32
     4b6:	00003317          	auipc	t1,0x3
     4ba:	33c30067          	jr	828(t1) # 37f2 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h2b088f06fe0c0eb7E>
     4be:	8522                	mv	a0,s0
     4c0:	00003097          	auipc	ra,0x3
     4c4:	91a080e7          	jalr	-1766(ra) # 2dda <_ZN4core3fmt9Formatter15debug_upper_hex17hfaedba9b5105966cE>
     4c8:	c919                	beqz	a0,4de <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hc899b377d5fb6a1bE+0x4c>
     4ca:	8526                	mv	a0,s1
     4cc:	85a2                	mv	a1,s0
     4ce:	64a2                	ld	s1,8(sp)
     4d0:	6442                	ld	s0,16(sp)
     4d2:	60e2                	ld	ra,24(sp)
     4d4:	6105                	addi	sp,sp,32
     4d6:	00003317          	auipc	t1,0x3
     4da:	3ac30067          	jr	940(t1) # 3882 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hc5ad538a9a7d4dd4E>
     4de:	8526                	mv	a0,s1
     4e0:	85a2                	mv	a1,s0
     4e2:	64a2                	ld	s1,8(sp)
     4e4:	6442                	ld	s0,16(sp)
     4e6:	60e2                	ld	ra,24(sp)
     4e8:	6105                	addi	sp,sp,32
     4ea:	00003317          	auipc	t1,0x3
     4ee:	5f830067          	jr	1528(t1) # 3ae2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h4cbd83af9447a2a8E>

00000000000004f2 <_ZN12tornado_user6shared15run_until_ready17he93e42bfed9e9094E>:
     4f2:	716d                	addi	sp,sp,-272
     4f4:	e606                	sd	ra,264(sp)
     4f6:	e222                	sd	s0,256(sp)
     4f8:	fda6                	sd	s1,248(sp)
     4fa:	f9ca                	sd	s2,240(sp)
     4fc:	f5ce                	sd	s3,232(sp)
     4fe:	f1d2                	sd	s4,224(sp)
     500:	edd6                	sd	s5,216(sp)
     502:	e9da                	sd	s6,208(sp)
     504:	e5de                	sd	s7,200(sp)
     506:	e1e2                	sd	s8,192(sp)
     508:	fd66                	sd	s9,184(sp)
     50a:	f96a                	sd	s10,176(sp)
     50c:	f56e                	sd	s11,168(sp)
     50e:	00053d83          	ld	s11,0(a0)
     512:	0005bc83          	ld	s9,0(a1)
     516:	02010993          	addi	s3,sp,32
     51a:	00898b93          	addi	s7,s3,8
     51e:	00063c03          	ld	s8,0(a2)
     522:	6288                	ld	a0,0(a3)
     524:	ec2a                	sd	a0,24(sp)

0000000000000526 <.LBB0_16>:
     526:	00001d17          	auipc	s10,0x1
     52a:	012d0d13          	addi	s10,s10,18 # 1538 <_ZN12tornado_user6shared16SharedTaskHandle13should_switch17h3346afc28a4cf301E>
     52e:	4a09                	li	s4,2
     530:	4a85                	li	s5,1
     532:	1884                	addi	s1,sp,112
     534:	09010913          	addi	s2,sp,144

0000000000000538 <.LBB0_17>:
     538:	00005517          	auipc	a0,0x5
     53c:	c9050513          	addi	a0,a0,-880 # 51c8 <anon.be5ea868b286bb36e8d2b81fd8475abb.0.llvm.4157917051624895175>
     540:	e82a                	sd	a0,16(sp)
     542:	0ff00b13          	li	s6,255
     546:	854e                	mv	a0,s3
     548:	85e6                	mv	a1,s9
     54a:	866a                	mv	a2,s10
     54c:	9d82                	jalr	s11
     54e:	7502                	ld	a0,32(sp)
     550:	0f450a63          	beq	a0,s4,644 <.LBB0_17+0x10c>
     554:	03550963          	beq	a0,s5,586 <.LBB0_17+0x4e>
     558:	008bb503          	ld	a0,8(s7)
     55c:	000bb583          	ld	a1,0(s7)
     560:	7462                	ld	s0,56(sp)
     562:	f0aa                	sd	a0,96(sp)
     564:	ecae                	sd	a1,88(sp)
     566:	8522                	mv	a0,s0
     568:	00001097          	auipc	ra,0x1
     56c:	b6a080e7          	jalr	-1174(ra) # 10d2 <_ZN12tornado_user4task8UserTask11is_sleeping17h921cc2599b53a729E>
     570:	c105                	beqz	a0,590 <.LBB0_17+0x58>
     572:	7506                	ld	a0,96(sp)
     574:	65e6                	ld	a1,88(sp)
     576:	ed2a                	sd	a0,152(sp)
     578:	e92e                	sd	a1,144(sp)
     57a:	f122                	sd	s0,160(sp)
     57c:	8526                	mv	a0,s1
     57e:	65e2                	ld	a1,24(sp)
     580:	864a                	mv	a2,s2
     582:	9c02                	jalr	s8
     584:	b7c9                	j	546 <.LBB0_17+0xe>
     586:	00001097          	auipc	ra,0x1
     58a:	002080e7          	jalr	2(ra) # 1588 <_ZN12tornado_user8do_yield17h86c358fa6dc094c1E>
     58e:	bf65                	j	546 <.LBB0_17+0xe>
     590:	e462                	sd	s8,8(sp)
     592:	008bb503          	ld	a0,8(s7)
     596:	000bb583          	ld	a1,0(s7)
     59a:	e4aa                	sd	a0,72(sp)
     59c:	ff040c13          	addi	s8,s0,-16
     5a0:	e0ae                	sd	a1,64(sp)
     5a2:	e8e2                	sd	s8,80(sp)
     5a4:	8522                	mv	a0,s0
     5a6:	00001097          	auipc	ra,0x1
     5aa:	bba080e7          	jalr	-1094(ra) # 1160 <_ZN12tornado_user4task8UserTask10mark_sleep17he16c82000448e43fE>
     5ae:	8522                	mv	a0,s0
     5b0:	65c2                	ld	a1,16(sp)
     5b2:	00002097          	auipc	ra,0x2
     5b6:	87e080e7          	jalr	-1922(ra) # 1e30 <_ZN4woke8WakerRef11new_unowned17h502a062306ae1e43E>
     5ba:	ecaa                	sd	a0,88(sp)
     5bc:	f0ae                	sd	a1,96(sp)
     5be:	08a8                	addi	a0,sp,88
     5c0:	00002097          	auipc	ra,0x2
     5c4:	872080e7          	jalr	-1934(ra) # 1e32 <_ZN58_$LT$woke..WakerRef$u20$as$u20$core..ops..deref..Deref$GT$5deref17h2fd191855ca923cdE>
     5c8:	f4aa                	sd	a0,104(sp)
     5ca:	018c0513          	addi	a0,s8,24
     5ce:	ffc57593          	andi	a1,a0,-4
     5d2:	00357613          	andi	a2,a0,3
     5d6:	060e                	slli	a2,a2,0x3
     5d8:	00cb16bb          	sllw	a3,s6,a2
     5dc:	00ca963b          	sllw	a2,s5,a2
     5e0:	1405a72f          	lr.w.aq	a4,(a1)
     5e4:	00d777b3          	and	a5,a4,a3
     5e8:	eb81                	bnez	a5,5f8 <.LBB0_17+0xc0>
     5ea:	00c747b3          	xor	a5,a4,a2
     5ee:	8ff5                	and	a5,a5,a3
     5f0:	8fb9                	xor	a5,a5,a4
     5f2:	18f5a7af          	sc.w	a5,a5,(a1)
     5f6:	f7ed                	bnez	a5,5e0 <.LBB0_17+0xa8>
     5f8:	00d77633          	and	a2,a4,a3
     5fc:	1602                	slli	a2,a2,0x20
     5fe:	9201                	srli	a2,a2,0x20
     600:	ca11                	beqz	a2,614 <.LBB0_17+0xdc>
     602:	00050603          	lb	a2,0(a0)
     606:	0ff67613          	andi	a2,a2,255
     60a:	fe65                	bnez	a2,602 <.LBB0_17+0xca>
     60c:	00351613          	slli	a2,a0,0x3
     610:	8a61                	andi	a2,a2,24
     612:	b7d9                	j	5d8 <.LBB0_17+0xa0>
     614:	028c3583          	ld	a1,40(s8)
     618:	020c3503          	ld	a0,32(s8)
     61c:	6d90                	ld	a2,24(a1)
     61e:	10ac                	addi	a1,sp,104
     620:	9602                	jalr	a2
     622:	0310000f          	fence	rw,w
     626:	000c0c23          	sb	zero,24(s8)
     62a:	cd1d                	beqz	a0,668 <.LBB0_17+0x130>
     62c:	6526                	ld	a0,72(sp)
     62e:	6586                	ld	a1,64(sp)
     630:	ed2a                	sd	a0,152(sp)
     632:	e92e                	sd	a1,144(sp)
     634:	f122                	sd	s0,160(sp)
     636:	1884                	addi	s1,sp,112
     638:	8526                	mv	a0,s1
     63a:	65e2                	ld	a1,24(sp)
     63c:	864a                	mv	a2,s2
     63e:	6c22                	ld	s8,8(sp)
     640:	9c02                	jalr	s8
     642:	b711                	j	546 <.LBB0_17+0xe>
     644:	4401                	li	s0,0
     646:	8522                	mv	a0,s0
     648:	85a6                	mv	a1,s1
     64a:	7daa                	ld	s11,168(sp)
     64c:	7d4a                	ld	s10,176(sp)
     64e:	7cea                	ld	s9,184(sp)
     650:	6c0e                	ld	s8,192(sp)
     652:	6bae                	ld	s7,200(sp)
     654:	6b4e                	ld	s6,208(sp)
     656:	6aee                	ld	s5,216(sp)
     658:	7a0e                	ld	s4,224(sp)
     65a:	79ae                	ld	s3,232(sp)
     65c:	794e                	ld	s2,240(sp)
     65e:	74ee                	ld	s1,248(sp)
     660:	6412                	ld	s0,256(sp)
     662:	60b2                	ld	ra,264(sp)
     664:	6151                	addi	sp,sp,272
     666:	8082                	ret
     668:	84ae                	mv	s1,a1
     66a:	4405                	li	s0,1
     66c:	40800533          	neg	a0,s0
     670:	02ac352f          	amoadd.d.rl	a0,a0,(s8)
     674:	fc8519e3          	bne	a0,s0,646 <.LBB0_17+0x10e>
     678:	0230000f          	fence	r,rw
     67c:	0888                	addi	a0,sp,80
     67e:	00000097          	auipc	ra,0x0
     682:	00a080e7          	jalr	10(ra) # 688 <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17haef57489042b0ba3E>
     686:	b7c1                	j	646 <.LBB0_17+0x10e>

0000000000000688 <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17haef57489042b0ba3E>:
     688:	1101                	addi	sp,sp,-32
     68a:	ec06                	sd	ra,24(sp)
     68c:	e822                	sd	s0,16(sp)
     68e:	e426                	sd	s1,8(sp)
     690:	842a                	mv	s0,a0
     692:	6104                	ld	s1,0(a0)
     694:	748c                	ld	a1,40(s1)
     696:	7088                	ld	a0,32(s1)
     698:	618c                	ld	a1,0(a1)
     69a:	9582                	jalr	a1
     69c:	7490                	ld	a2,40(s1)
     69e:	660c                	ld	a1,8(a2)
     6a0:	c599                	beqz	a1,6ae <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17haef57489042b0ba3E+0x26>
     6a2:	7088                	ld	a0,32(s1)
     6a4:	6a10                	ld	a2,16(a2)
     6a6:	00000097          	auipc	ra,0x0
     6aa:	464080e7          	jalr	1124(ra) # b0a <__rust_dealloc>
     6ae:	6008                	ld	a0,0(s0)
     6b0:	55fd                	li	a1,-1
     6b2:	02b50863          	beq	a0,a1,6e2 <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17haef57489042b0ba3E+0x5a>
     6b6:	00850593          	addi	a1,a0,8
     6ba:	4605                	li	a2,1
     6bc:	40c006b3          	neg	a3,a2
     6c0:	02d5b5af          	amoadd.d.rl	a1,a3,(a1)
     6c4:	00c59f63          	bne	a1,a2,6e2 <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17haef57489042b0ba3E+0x5a>
     6c8:	03800593          	li	a1,56
     6cc:	4621                	li	a2,8
     6ce:	0230000f          	fence	r,rw
     6d2:	64a2                	ld	s1,8(sp)
     6d4:	6442                	ld	s0,16(sp)
     6d6:	60e2                	ld	ra,24(sp)
     6d8:	6105                	addi	sp,sp,32
     6da:	00000317          	auipc	t1,0x0
     6de:	43030067          	jr	1072(t1) # b0a <__rust_dealloc>
     6e2:	64a2                	ld	s1,8(sp)
     6e4:	6442                	ld	s0,16(sp)
     6e6:	60e2                	ld	ra,24(sp)
     6e8:	6105                	addi	sp,sp,32
     6ea:	8082                	ret

00000000000006ec <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17h24bb2609aa7c8facE.llvm.16017524911331216699>:
     6ec:	715d                	addi	sp,sp,-80
     6ee:	e486                	sd	ra,72(sp)
     6f0:	e0a2                	sd	s0,64(sp)
     6f2:	fc26                	sd	s1,56(sp)
     6f4:	f84a                	sd	s2,48(sp)
     6f6:	842a                	mv	s0,a0
     6f8:	6108                	ld	a0,0(a0)
     6fa:	640c                	ld	a1,8(s0)
     6fc:	6c04                	ld	s1,24(s0)
     6fe:	40a58533          	sub	a0,a1,a0
     702:	fff48593          	addi	a1,s1,-1
     706:	8d6d                	and	a0,a0,a1
     708:	40a48533          	sub	a0,s1,a0
     70c:	4585                	li	a1,1
     70e:	0cb51863          	bne	a0,a1,7de <.LBB0_18+0x38>
     712:	c0c9                	beqz	s1,794 <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17h24bb2609aa7c8facE.llvm.16017524911331216699+0xa8>
     714:	00948533          	add	a0,s1,s1
     718:	0c956a63          	bltu	a0,s1,7ec <.LBB0_18+0x46>
     71c:	4601                	li	a2,0
     71e:	4905                	li	s2,1
     720:	03d91593          	slli	a1,s2,0x3d
     724:	15fd                	addi	a1,a1,-1
     726:	8de9                	and	a1,a1,a0
     728:	8da9                	xor	a1,a1,a0
     72a:	00b036b3          	snez	a3,a1
     72e:	00351593          	slli	a1,a0,0x3
     732:	4521                	li	a0,8
     734:	e291                	bnez	a3,738 <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17h24bb2609aa7c8facE.llvm.16017524911331216699+0x4c>
     736:	4621                	li	a2,8
     738:	6814                	ld	a3,16(s0)
     73a:	00349713          	slli	a4,s1,0x3
     73e:	ec36                	sd	a3,24(sp)
     740:	f03a                	sd	a4,32(sp)
     742:	f42a                	sd	a0,40(sp)
     744:	850a                	mv	a0,sp
     746:	0834                	addi	a3,sp,24
     748:	00000097          	auipc	ra,0x0
     74c:	924080e7          	jalr	-1756(ra) # 6c <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351>
     750:	6602                	ld	a2,0(sp)
     752:	6522                	ld	a0,8(sp)
     754:	65c2                	ld	a1,16(sp)
     756:	09260a63          	beq	a2,s2,7ea <.LBB0_18+0x44>
     75a:	e808                	sd	a0,16(s0)
     75c:	0035d513          	srli	a0,a1,0x3
     760:	ec08                	sd	a0,24(s0)
     762:	00149593          	slli	a1,s1,0x1
     766:	02b51c63          	bne	a0,a1,79e <.LBB0_17>
     76a:	600c                	ld	a1,0(s0)
     76c:	6410                	ld	a2,8(s0)
     76e:	06b67863          	bgeu	a2,a1,7de <.LBB0_18+0x38>
     772:	40b486b3          	sub	a3,s1,a1
     776:	04d67363          	bgeu	a2,a3,7bc <.LBB0_18+0x16>
     77a:	680c                	ld	a1,16(s0)
     77c:	00349513          	slli	a0,s1,0x3
     780:	952e                	add	a0,a0,a1
     782:	060e                	slli	a2,a2,0x3
     784:	00004097          	auipc	ra,0x4
     788:	874080e7          	jalr	-1932(ra) # 3ff8 <memcpy>
     78c:	6408                	ld	a0,8(s0)
     78e:	9526                	add	a0,a0,s1
     790:	e408                	sd	a0,8(s0)
     792:	a0b1                	j	7de <.LBB0_18+0x38>
     794:	4501                	li	a0,0
     796:	00149593          	slli	a1,s1,0x1
     79a:	fcb508e3          	beq	a0,a1,76a <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17h24bb2609aa7c8facE.llvm.16017524911331216699+0x7e>

000000000000079e <.LBB0_17>:
     79e:	00005517          	auipc	a0,0x5
     7a2:	98250513          	addi	a0,a0,-1662 # 5120 <.Lanon.2cd93b564825415fcd61733825743066.0>

00000000000007a6 <.LBB0_18>:
     7a6:	00005617          	auipc	a2,0x5
     7aa:	a0a60613          	addi	a2,a2,-1526 # 51b0 <.Lanon.2cd93b564825415fcd61733825743066.2>
     7ae:	02b00593          	li	a1,43
     7b2:	00001097          	auipc	ra,0x1
     7b6:	790080e7          	jalr	1936(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
     7ba:	0000                	unimp
     7bc:	6810                	ld	a2,16(s0)
     7be:	40d504b3          	sub	s1,a0,a3
     7c2:	00359513          	slli	a0,a1,0x3
     7c6:	00a605b3          	add	a1,a2,a0
     7ca:	00349513          	slli	a0,s1,0x3
     7ce:	9532                	add	a0,a0,a2
     7d0:	00369613          	slli	a2,a3,0x3
     7d4:	00004097          	auipc	ra,0x4
     7d8:	824080e7          	jalr	-2012(ra) # 3ff8 <memcpy>
     7dc:	e004                	sd	s1,0(s0)
     7de:	7942                	ld	s2,48(sp)
     7e0:	74e2                	ld	s1,56(sp)
     7e2:	6406                	ld	s0,64(sp)
     7e4:	60a6                	ld	ra,72(sp)
     7e6:	6161                	addi	sp,sp,80
     7e8:	8082                	ret
     7ea:	e591                	bnez	a1,7f6 <.LBB0_18+0x50>
     7ec:	00001097          	auipc	ra,0x1
     7f0:	676080e7          	jalr	1654(ra) # 1e62 <_ZN5alloc7raw_vec17capacity_overflow17hb114381a505af03eE>
     7f4:	0000                	unimp
     7f6:	00001097          	auipc	ra,0x1
     7fa:	650080e7          	jalr	1616(ra) # 1e46 <_ZN5alloc5alloc18handle_alloc_error17h0809b9ba7eebe66bE>
	...

0000000000000800 <_ZN4woke12drop_arc_raw17hd4f9b02fae2d6a82E.llvm.4157917051624895175>:
     800:	1141                	addi	sp,sp,-16
     802:	e406                	sd	ra,8(sp)
     804:	1541                	addi	a0,a0,-16
     806:	e02a                	sd	a0,0(sp)
     808:	4585                	li	a1,1
     80a:	40b00633          	neg	a2,a1
     80e:	02c5352f          	amoadd.d.rl	a0,a2,(a0)
     812:	00b51963          	bne	a0,a1,824 <_ZN4woke12drop_arc_raw17hd4f9b02fae2d6a82E.llvm.4157917051624895175+0x24>
     816:	0230000f          	fence	r,rw
     81a:	850a                	mv	a0,sp
     81c:	00000097          	auipc	ra,0x0
     820:	e6c080e7          	jalr	-404(ra) # 688 <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17haef57489042b0ba3E>
     824:	60a2                	ld	ra,8(sp)
     826:	0141                	addi	sp,sp,16
     828:	8082                	ret

000000000000082a <_ZN4woke12wake_arc_raw17h6d477a01c2f136e4E.llvm.4157917051624895175>:
     82a:	1141                	addi	sp,sp,-16
     82c:	e406                	sd	ra,8(sp)
     82e:	1541                	addi	a0,a0,-16
     830:	e02a                	sd	a0,0(sp)
     832:	850a                	mv	a0,sp
     834:	00001097          	auipc	ra,0x1
     838:	9b6080e7          	jalr	-1610(ra) # 11ea <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE>
     83c:	6502                	ld	a0,0(sp)
     83e:	4585                	li	a1,1
     840:	40b00633          	neg	a2,a1
     844:	02c5352f          	amoadd.d.rl	a0,a2,(a0)
     848:	00b51963          	bne	a0,a1,85a <_ZN4woke12wake_arc_raw17h6d477a01c2f136e4E.llvm.4157917051624895175+0x30>
     84c:	0230000f          	fence	r,rw
     850:	850a                	mv	a0,sp
     852:	00000097          	auipc	ra,0x0
     856:	e36080e7          	jalr	-458(ra) # 688 <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17haef57489042b0ba3E>
     85a:	60a2                	ld	ra,8(sp)
     85c:	0141                	addi	sp,sp,16
     85e:	8082                	ret

0000000000000860 <_ZN4woke13clone_arc_raw17h9d673293048caf78E.llvm.4157917051624895175>:
     860:	ff050593          	addi	a1,a0,-16
     864:	4605                	li	a2,1
     866:	00c5b5af          	amoadd.d	a1,a2,(a1)
     86a:	567d                	li	a2,-1
     86c:	00b65763          	bge	a2,a1,87a <.LBB3_3+0xa>

0000000000000870 <.LBB3_3>:
     870:	00005597          	auipc	a1,0x5
     874:	95858593          	addi	a1,a1,-1704 # 51c8 <anon.be5ea868b286bb36e8d2b81fd8475abb.0.llvm.4157917051624895175>
     878:	8082                	ret
     87a:	0000                	unimp
	...

000000000000087e <_ZN4woke19wake_by_ref_arc_raw17h4dceec1db4de0323E.llvm.4157917051624895175>:
     87e:	1141                	addi	sp,sp,-16
     880:	e406                	sd	ra,8(sp)
     882:	1541                	addi	a0,a0,-16
     884:	e02a                	sd	a0,0(sp)
     886:	850a                	mv	a0,sp
     888:	00001097          	auipc	ra,0x1
     88c:	962080e7          	jalr	-1694(ra) # 11ea <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE>
     890:	60a2                	ld	ra,8(sp)
     892:	0141                	addi	sp,sp,16
     894:	8082                	ret

0000000000000896 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h249276c6913a58b4E>:
     896:	7139                	addi	sp,sp,-64
     898:	fc06                	sd	ra,56(sp)
     89a:	f822                	sd	s0,48(sp)
     89c:	f426                	sd	s1,40(sp)
     89e:	6108                	ld	a0,0(a0)
     8a0:	6110                	ld	a2,0(a0)
     8a2:	4685                	li	a3,1
     8a4:	02d61b63          	bne	a2,a3,8da <.LBB0_6>
     8a8:	00850493          	addi	s1,a0,8

00000000000008ac <.LBB0_4>:
     8ac:	00005617          	auipc	a2,0x5
     8b0:	94060613          	addi	a2,a2,-1728 # 51ec <anon.be5ea868b286bb36e8d2b81fd8475abb.0.llvm.4157917051624895175+0x24>
     8b4:	0020                	addi	s0,sp,8
     8b6:	4691                	li	a3,4
     8b8:	8522                	mv	a0,s0
     8ba:	00002097          	auipc	ra,0x2
     8be:	52c080e7          	jalr	1324(ra) # 2de6 <_ZN4core3fmt9Formatter11debug_tuple17h5b19d3349176789bE>
     8c2:	f026                	sd	s1,32(sp)

00000000000008c4 <.LBB0_5>:
     8c4:	00005617          	auipc	a2,0x5
     8c8:	93460613          	addi	a2,a2,-1740 # 51f8 <.Lanon.a5be637f67c858a9fd1543754584150a.1>
     8cc:	100c                	addi	a1,sp,32
     8ce:	8522                	mv	a0,s0
     8d0:	00002097          	auipc	ra,0x2
     8d4:	91c080e7          	jalr	-1764(ra) # 21ec <_ZN4core3fmt8builders10DebugTuple5field17he8ff51e3f51afcb6E>
     8d8:	a821                	j	8f0 <.LBB0_6+0x16>

00000000000008da <.LBB0_6>:
     8da:	00005617          	auipc	a2,0x5
     8de:	91a60613          	addi	a2,a2,-1766 # 51f4 <anon.be5ea868b286bb36e8d2b81fd8475abb.0.llvm.4157917051624895175+0x2c>
     8e2:	0020                	addi	s0,sp,8
     8e4:	4691                	li	a3,4
     8e6:	8522                	mv	a0,s0
     8e8:	00002097          	auipc	ra,0x2
     8ec:	4fe080e7          	jalr	1278(ra) # 2de6 <_ZN4core3fmt9Formatter11debug_tuple17h5b19d3349176789bE>
     8f0:	8522                	mv	a0,s0
     8f2:	00002097          	auipc	ra,0x2
     8f6:	9ee080e7          	jalr	-1554(ra) # 22e0 <_ZN4core3fmt8builders10DebugTuple6finish17h9892d8aebff080cbE>
     8fa:	74a2                	ld	s1,40(sp)
     8fc:	7442                	ld	s0,48(sp)
     8fe:	70e2                	ld	ra,56(sp)
     900:	6121                	addi	sp,sp,64
     902:	8082                	ret

0000000000000904 <_ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17h9b3b6ccf2262c683E>:
     904:	8082                	ret

0000000000000906 <_ZN4core3ptr51drop_in_place$LT$$RF$alloc..vec..Vec$LT$i32$GT$$GT$17h54a8bb50ad6b43e7E.llvm.1894474860202936077>:
     906:	8082                	ret

0000000000000908 <_ZN4core9panicking13assert_failed17h89f31df7e7d23afbE>:
     908:	7139                	addi	sp,sp,-64
     90a:	883a                	mv	a6,a4
     90c:	7698                	ld	a4,40(a3)
     90e:	729c                	ld	a5,32(a3)
     910:	e02e                	sd	a1,0(sp)
     912:	e432                	sd	a2,8(sp)
     914:	fc3a                	sd	a4,56(sp)
     916:	f83e                	sd	a5,48(sp)
     918:	6e8c                	ld	a1,24(a3)
     91a:	6a90                	ld	a2,16(a3)
     91c:	6698                	ld	a4,8(a3)
     91e:	6294                	ld	a3,0(a3)
     920:	f42e                	sd	a1,40(sp)
     922:	f032                	sd	a2,32(sp)
     924:	ec3a                	sd	a4,24(sp)
     926:	e836                	sd	a3,16(sp)

0000000000000928 <.LBB1_1>:
     928:	00005617          	auipc	a2,0x5
     92c:	8f060613          	addi	a2,a2,-1808 # 5218 <anon.780e3732ec4a8f2c2a9317f7c3bfa294.0.llvm.1894474860202936077>
     930:	858a                	mv	a1,sp
     932:	0034                	addi	a3,sp,8
     934:	081c                	addi	a5,sp,16
     936:	8732                	mv	a4,a2
     938:	00001097          	auipc	ra,0x1
     93c:	69e080e7          	jalr	1694(ra) # 1fd6 <_ZN4core9panicking13assert_failed5inner17h080205f7502c6bf2E>
	...

0000000000000942 <_ZN4core9panicking13assert_failed17hb943cda24c27692cE>:
     942:	7139                	addi	sp,sp,-64
     944:	883a                	mv	a6,a4
     946:	7698                	ld	a4,40(a3)
     948:	729c                	ld	a5,32(a3)
     94a:	e02e                	sd	a1,0(sp)
     94c:	e432                	sd	a2,8(sp)
     94e:	fc3a                	sd	a4,56(sp)
     950:	f83e                	sd	a5,48(sp)
     952:	6e8c                	ld	a1,24(a3)
     954:	6a90                	ld	a2,16(a3)
     956:	6698                	ld	a4,8(a3)
     958:	6294                	ld	a3,0(a3)
     95a:	f42e                	sd	a1,40(sp)
     95c:	f032                	sd	a2,32(sp)
     95e:	ec3a                	sd	a4,24(sp)
     960:	e836                	sd	a3,16(sp)

0000000000000962 <.LBB2_1>:
     962:	00005617          	auipc	a2,0x5
     966:	8d660613          	addi	a2,a2,-1834 # 5238 <anon.780e3732ec4a8f2c2a9317f7c3bfa294.1.llvm.1894474860202936077>
     96a:	858a                	mv	a1,sp
     96c:	0034                	addi	a3,sp,8
     96e:	081c                	addi	a5,sp,16
     970:	8732                	mv	a4,a2
     972:	00001097          	auipc	ra,0x1
     976:	664080e7          	jalr	1636(ra) # 1fd6 <_ZN4core9panicking13assert_failed5inner17h080205f7502c6bf2E>
	...

000000000000097c <_ZN12tornado_user7excutor5spawn17h7b2a2b919cdc5b7aE>:
     97c:	715d                	addi	sp,sp,-80
     97e:	e486                	sd	ra,72(sp)
     980:	e0a2                	sd	s0,64(sp)
     982:	fc26                	sd	s1,56(sp)
     984:	f84a                	sd	s2,48(sp)
     986:	84aa                	mv	s1,a0

0000000000000988 <.LBB0_11>:
     988:	00005517          	auipc	a0,0x5
     98c:	95050513          	addi	a0,a0,-1712 # 52d8 <_ZN12tornado_user7excutor8EXECUTOR17hf6915d4efcbd1205E>
     990:	00000097          	auipc	ra,0x0
     994:	510080e7          	jalr	1296(ra) # ea0 <_ZN75_$LT$tornado_user..excutor..EXECUTOR$u20$as$u20$core..ops..deref..Deref$GT$5deref17hee4014ce0b3f475dE>
     998:	842a                	mv	s0,a0
     99a:	6c88                	ld	a0,24(s1)
     99c:	688c                	ld	a1,16(s1)
     99e:	6490                	ld	a2,8(s1)
     9a0:	6094                	ld	a3,0(s1)
     9a2:	f02a                	sd	a0,32(sp)
     9a4:	ec2e                	sd	a1,24(sp)
     9a6:	e832                	sd	a2,16(sp)
     9a8:	e436                	sd	a3,8(sp)
     9aa:	02000513          	li	a0,32
     9ae:	45a1                	li	a1,8
     9b0:	00000097          	auipc	ra,0x0
     9b4:	152080e7          	jalr	338(ra) # b02 <__rust_alloc>
     9b8:	12050963          	beqz	a0,aea <.LBB0_12+0xba>
     9bc:	892a                	mv	s2,a0
     9be:	7502                	ld	a0,32(sp)
     9c0:	65e2                	ld	a1,24(sp)
     9c2:	6642                	ld	a2,16(sp)
     9c4:	66a2                	ld	a3,8(sp)
     9c6:	00a93c23          	sd	a0,24(s2)
     9ca:	00b93823          	sd	a1,16(s2)
     9ce:	00c93423          	sd	a2,8(s2)
     9d2:	00d93023          	sd	a3,0(s2)
     9d6:	03000513          	li	a0,48
     9da:	45a1                	li	a1,8
     9dc:	00000097          	auipc	ra,0x0
     9e0:	126080e7          	jalr	294(ra) # b02 <__rust_alloc>
     9e4:	10050663          	beqz	a0,af0 <.LBB0_12+0xc0>
     9e8:	84aa                	mv	s1,a0
     9ea:	4505                	li	a0,1
     9ec:	e088                	sd	a0,0(s1)
     9ee:	e488                	sd	a0,8(s1)
     9f0:	00810583          	lb	a1,8(sp)
     9f4:	00910603          	lb	a2,9(sp)
     9f8:	00a10683          	lb	a3,10(sp)
     9fc:	00048823          	sb	zero,16(s1)
     a00:	00b488a3          	sb	a1,17(s1)
     a04:	00c48923          	sb	a2,18(s1)
     a08:	00d489a3          	sb	a3,19(s1)
     a0c:	00b10583          	lb	a1,11(sp)
     a10:	00c10603          	lb	a2,12(sp)
     a14:	00d10683          	lb	a3,13(sp)
     a18:	00e10703          	lb	a4,14(sp)
     a1c:	00b48a23          	sb	a1,20(s1)
     a20:	00c48aa3          	sb	a2,21(s1)
     a24:	00d48b23          	sb	a3,22(s1)
     a28:	00e48ba3          	sb	a4,23(s1)
     a2c:	0124bc23          	sd	s2,24(s1)

0000000000000a30 <.LBB0_12>:
     a30:	00005597          	auipc	a1,0x5
     a34:	82858593          	addi	a1,a1,-2008 # 5258 <anon.12908af6f985cbf1ec18d41b8c469eb6.0.llvm.10739002475286810050>
     a38:	f08c                	sd	a1,32(s1)
     a3a:	02a11583          	lh	a1,42(sp)
     a3e:	02c11603          	lh	a2,44(sp)
     a42:	02e11683          	lh	a3,46(sp)
     a46:	02049423          	sh	zero,40(s1)
     a4a:	02b49523          	sh	a1,42(s1)
     a4e:	02c49623          	sh	a2,44(s1)
     a52:	02d49723          	sh	a3,46(s1)
     a56:	ffc47593          	andi	a1,s0,-4
     a5a:	00341613          	slli	a2,s0,0x3
     a5e:	01867693          	andi	a3,a2,24
     a62:	0ff00613          	li	a2,255
     a66:	00d6163b          	sllw	a2,a2,a3
     a6a:	00d516bb          	sllw	a3,a0,a3
     a6e:	1405a72f          	lr.w.aq	a4,(a1)
     a72:	00c777b3          	and	a5,a4,a2
     a76:	eb81                	bnez	a5,a86 <.LBB0_12+0x56>
     a78:	00d747b3          	xor	a5,a4,a3
     a7c:	8ff1                	and	a5,a5,a2
     a7e:	8fb9                	xor	a5,a5,a4
     a80:	18f5a7af          	sc.w	a5,a5,(a1)
     a84:	f7ed                	bnez	a5,a6e <.LBB0_12+0x3e>
     a86:	8f71                	and	a4,a4,a2
     a88:	1702                	slli	a4,a4,0x20
     a8a:	9301                	srli	a4,a4,0x20
     a8c:	c719                	beqz	a4,a9a <.LBB0_12+0x6a>
     a8e:	00040703          	lb	a4,0(s0)
     a92:	0ff77713          	andi	a4,a4,255
     a96:	ff65                	bnez	a4,a8e <.LBB0_12+0x5e>
     a98:	bfd9                	j	a6e <.LBB0_12+0x3e>
     a9a:	6410                	ld	a2,8(s0)
     a9c:	680c                	ld	a1,16(s0)
     a9e:	7014                	ld	a3,32(s0)
     aa0:	40c58733          	sub	a4,a1,a2
     aa4:	fff68613          	addi	a2,a3,-1
     aa8:	8f71                	and	a4,a4,a2
     aaa:	8e99                	sub	a3,a3,a4
     aac:	00a69c63          	bne	a3,a0,ac4 <.LBB0_12+0x94>
     ab0:	00840513          	addi	a0,s0,8
     ab4:	00000097          	auipc	ra,0x0
     ab8:	c38080e7          	jalr	-968(ra) # 6ec <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17h24bb2609aa7c8facE.llvm.16017524911331216699>
     abc:	7008                	ld	a0,32(s0)
     abe:	680c                	ld	a1,16(s0)
     ac0:	fff50613          	addi	a2,a0,-1
     ac4:	00158513          	addi	a0,a1,1
     ac8:	6c14                	ld	a3,24(s0)
     aca:	8d71                	and	a0,a0,a2
     acc:	e808                	sd	a0,16(s0)
     ace:	00359513          	slli	a0,a1,0x3
     ad2:	9536                	add	a0,a0,a3
     ad4:	e104                	sd	s1,0(a0)
     ad6:	0310000f          	fence	rw,w
     ada:	00040023          	sb	zero,0(s0)
     ade:	7942                	ld	s2,48(sp)
     ae0:	74e2                	ld	s1,56(sp)
     ae2:	6406                	ld	s0,64(sp)
     ae4:	60a6                	ld	ra,72(sp)
     ae6:	6161                	addi	sp,sp,80
     ae8:	8082                	ret
     aea:	02000513          	li	a0,32
     aee:	a019                	j	af4 <.LBB0_12+0xc4>
     af0:	03000513          	li	a0,48
     af4:	45a1                	li	a1,8
     af6:	00001097          	auipc	ra,0x1
     afa:	350080e7          	jalr	848(ra) # 1e46 <_ZN5alloc5alloc18handle_alloc_error17h0809b9ba7eebe66bE>
	...

0000000000000b00 <_ZN4core3ptr47drop_in_place$LT$user_task..FibonacciFuture$GT$17h11c90872ba4eff3cE.llvm.10739002475286810050>:
     b00:	8082                	ret

0000000000000b02 <__rust_alloc>:
     b02:	00001317          	auipc	t1,0x1
     b06:	a9630067          	jr	-1386(t1) # 1598 <__rg_alloc>

0000000000000b0a <__rust_dealloc>:
     b0a:	00001317          	auipc	t1,0x1
     b0e:	aa630067          	jr	-1370(t1) # 15b0 <__rg_dealloc>

0000000000000b12 <__rust_realloc>:
     b12:	00001317          	auipc	t1,0x1
     b16:	ab830067          	jr	-1352(t1) # 15ca <__rg_realloc>

0000000000000b1a <__rust_alloc_zeroed>:
     b1a:	00001317          	auipc	t1,0x1
     b1e:	b1830067          	jr	-1256(t1) # 1632 <__rg_alloc_zeroed>

0000000000000b22 <__rust_alloc_error_handler>:
     b22:	00001317          	auipc	t1,0x1
     b26:	33230067          	jr	818(t1) # 1e54 <__rg_oom>

0000000000000b2a <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE>:
     b2a:	7139                	addi	sp,sp,-64
     b2c:	fc06                	sd	ra,56(sp)
     b2e:	f822                	sd	s0,48(sp)
     b30:	f426                	sd	s1,40(sp)
     b32:	f04a                	sd	s2,32(sp)
     b34:	ec4e                	sd	s3,24(sp)
     b36:	e852                	sd	s4,16(sp)
     b38:	e456                	sd	s5,8(sp)
     b3a:	e05a                	sd	s6,0(sp)
     b3c:	842a                	mv	s0,a0
     b3e:	9971                	andi	a0,a0,-4
     b40:	00341593          	slli	a1,s0,0x3
     b44:	0185f613          	andi	a2,a1,24
     b48:	0ff00593          	li	a1,255
     b4c:	00c595bb          	sllw	a1,a1,a2
     b50:	4685                	li	a3,1
     b52:	00c6963b          	sllw	a2,a3,a2
     b56:	140526af          	lr.w.aq	a3,(a0)
     b5a:	00b6f733          	and	a4,a3,a1
     b5e:	eb01                	bnez	a4,b6e <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0x44>
     b60:	00c6c733          	xor	a4,a3,a2
     b64:	8f6d                	and	a4,a4,a1
     b66:	8f35                	xor	a4,a4,a3
     b68:	18e5272f          	sc.w	a4,a4,(a0)
     b6c:	f76d                	bnez	a4,b56 <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0x2c>
     b6e:	8eed                	and	a3,a3,a1
     b70:	1682                	slli	a3,a3,0x20
     b72:	9281                	srli	a3,a3,0x20
     b74:	c699                	beqz	a3,b82 <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0x58>
     b76:	00040683          	lb	a3,0(s0)
     b7a:	0ff6f693          	andi	a3,a3,255
     b7e:	fee5                	bnez	a3,b76 <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0x4c>
     b80:	bfd9                	j	b56 <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0x2c>
     b82:	6408                	ld	a0,8(s0)
     b84:	680c                	ld	a1,16(s0)
     b86:	7010                	ld	a2,32(s0)
     b88:	40a586b3          	sub	a3,a1,a0
     b8c:	167d                	addi	a2,a2,-1
     b8e:	00d679b3          	and	s3,a2,a3
     b92:	0e098563          	beqz	s3,c7c <.LBB2_20+0x16>
     b96:	0ca58463          	beq	a1,a0,c5e <.LBB2_19>
     b9a:	00840913          	addi	s2,s0,8
     b9e:	4485                	li	s1,1
     ba0:	0ff00a93          	li	s5,255
     ba4:	4b05                	li	s6,1
     ba6:	700c                	ld	a1,32(s0)
     ba8:	00150613          	addi	a2,a0,1
     bac:	15fd                	addi	a1,a1,-1
     bae:	6c14                	ld	a3,24(s0)
     bb0:	8df1                	and	a1,a1,a2
     bb2:	e40c                	sd	a1,8(s0)
     bb4:	050e                	slli	a0,a0,0x3
     bb6:	9536                	add	a0,a0,a3
     bb8:	00053a03          	ld	s4,0(a0)
     bbc:	0a0a0163          	beqz	s4,c5e <.LBB2_19>
     bc0:	028a0513          	addi	a0,s4,40
     bc4:	ffc57593          	andi	a1,a0,-4
     bc8:	00357613          	andi	a2,a0,3
     bcc:	060e                	slli	a2,a2,0x3
     bce:	00ca96bb          	sllw	a3,s5,a2
     bd2:	00c4963b          	sllw	a2,s1,a2
     bd6:	1405a72f          	lr.w.aq	a4,(a1)
     bda:	00d777b3          	and	a5,a4,a3
     bde:	eb81                	bnez	a5,bee <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0xc4>
     be0:	00c747b3          	xor	a5,a4,a2
     be4:	8ff5                	and	a5,a5,a3
     be6:	8fb9                	xor	a5,a5,a4
     be8:	18f5a7af          	sc.w	a5,a5,(a1)
     bec:	f7ed                	bnez	a5,bd6 <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0xac>
     bee:	00d77633          	and	a2,a4,a3
     bf2:	1602                	slli	a2,a2,0x20
     bf4:	9201                	srli	a2,a2,0x20
     bf6:	ca11                	beqz	a2,c0a <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0xe0>
     bf8:	00050603          	lb	a2,0(a0)
     bfc:	0ff67613          	andi	a2,a2,255
     c00:	fe65                	bnez	a2,bf8 <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0xce>
     c02:	00351613          	slli	a2,a0,0x3
     c06:	8a61                	andi	a2,a2,24
     c08:	b7d9                	j	bce <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0xa4>
     c0a:	00154583          	lbu	a1,1(a0)
     c0e:	0310000f          	fence	rw,w
     c12:	00050023          	sb	zero,0(a0)
     c16:	c5a5                	beqz	a1,c7e <.LBB2_20+0x18>
     c18:	640c                	ld	a1,8(s0)
     c1a:	6808                	ld	a0,16(s0)
     c1c:	7010                	ld	a2,32(s0)
     c1e:	40b506b3          	sub	a3,a0,a1
     c22:	fff60593          	addi	a1,a2,-1
     c26:	8eed                	and	a3,a3,a1
     c28:	8e15                	sub	a2,a2,a3
     c2a:	00961a63          	bne	a2,s1,c3e <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0x114>
     c2e:	854a                	mv	a0,s2
     c30:	00000097          	auipc	ra,0x0
     c34:	64c080e7          	jalr	1612(ra) # 127c <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17hdc31b30d85414892E.llvm.227590070318396806>
     c38:	700c                	ld	a1,32(s0)
     c3a:	6808                	ld	a0,16(s0)
     c3c:	15fd                	addi	a1,a1,-1
     c3e:	00150613          	addi	a2,a0,1
     c42:	6c14                	ld	a3,24(s0)
     c44:	8df1                	and	a1,a1,a2
     c46:	e80c                	sd	a1,16(s0)
     c48:	050e                	slli	a0,a0,0x3
     c4a:	9536                	add	a0,a0,a3
     c4c:	01453023          	sd	s4,0(a0)
     c50:	033b0663          	beq	s6,s3,c7c <.LBB2_20+0x16>
     c54:	6408                	ld	a0,8(s0)
     c56:	680c                	ld	a1,16(s0)
     c58:	0b05                	addi	s6,s6,1
     c5a:	f4b516e3          	bne	a0,a1,ba6 <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE+0x7c>

0000000000000c5e <.LBB2_19>:
     c5e:	00004517          	auipc	a0,0x4
     c62:	61a50513          	addi	a0,a0,1562 # 5278 <.Lanon.5ffa1a0ec8fe4778f9c8f909c00eaf94.0>

0000000000000c66 <.LBB2_20>:
     c66:	00004617          	auipc	a2,0x4
     c6a:	65a60613          	addi	a2,a2,1626 # 52c0 <.Lanon.5ffa1a0ec8fe4778f9c8f909c00eaf94.2>
     c6e:	02b00593          	li	a1,43
     c72:	00001097          	auipc	ra,0x1
     c76:	2d0080e7          	jalr	720(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
     c7a:	0000                	unimp
     c7c:	4a01                	li	s4,0
     c7e:	0310000f          	fence	rw,w
     c82:	00040023          	sb	zero,0(s0)
     c86:	8552                	mv	a0,s4
     c88:	6b02                	ld	s6,0(sp)
     c8a:	6aa2                	ld	s5,8(sp)
     c8c:	6a42                	ld	s4,16(sp)
     c8e:	69e2                	ld	s3,24(sp)
     c90:	7902                	ld	s2,32(sp)
     c92:	74a2                	ld	s1,40(sp)
     c94:	7442                	ld	s0,48(sp)
     c96:	70e2                	ld	ra,56(sp)
     c98:	6121                	addi	sp,sp,64
     c9a:	8082                	ret

0000000000000c9c <_ZN12tornado_user7excutor8try_join17h027004220a5c13bdE>:
     c9c:	7175                	addi	sp,sp,-144
     c9e:	e506                	sd	ra,136(sp)
     ca0:	e122                	sd	s0,128(sp)
     ca2:	fca6                	sd	s1,120(sp)
     ca4:	f8ca                	sd	s2,112(sp)
     ca6:	f4ce                	sd	s3,104(sp)
     ca8:	f0d2                	sd	s4,96(sp)
     caa:	ecd6                	sd	s5,88(sp)
     cac:	e8da                	sd	s6,80(sp)
     cae:	e4de                	sd	s7,72(sp)
     cb0:	e0e2                	sd	s8,64(sp)
     cb2:	fc66                	sd	s9,56(sp)
     cb4:	f86a                	sd	s10,48(sp)
     cb6:	f46e                	sd	s11,40(sp)

0000000000000cb8 <.LBB4_20>:
     cb8:	00006517          	auipc	a0,0x6
     cbc:	34850513          	addi	a0,a0,840 # 7000 <_ZN75_$LT$tornado_user..excutor..EXECUTOR$u20$as$u20$core..ops..deref..Deref$GT$5deref11__stability4LAZY17h53ae8f4cd44a05c3E>
     cc0:	00001097          	auipc	ra,0x1
     cc4:	9ae080e7          	jalr	-1618(ra) # 166e <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE>
     cc8:	842a                	mv	s0,a0
     cca:	00000097          	auipc	ra,0x0
     cce:	e60080e7          	jalr	-416(ra) # b2a <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE>
     cd2:	18050563          	beqz	a0,e5c <.LBB4_21+0x17c>
     cd6:	00840913          	addi	s2,s0,8
     cda:	0ff00b13          	li	s6,255
     cde:	4b85                	li	s7,1

0000000000000ce0 <.LBB4_21>:
     ce0:	00004a97          	auipc	s5,0x4
     ce4:	5f8a8a93          	addi	s5,s5,1528 # 52d8 <_ZN12tornado_user7excutor8EXECUTOR17hf6915d4efcbd1205E>
     ce8:	01010993          	addi	s3,sp,16
     cec:	02010a13          	addi	s4,sp,32
     cf0:	ffc47c13          	andi	s8,s0,-4
     cf4:	00341593          	slli	a1,s0,0x3
     cf8:	89e1                	andi	a1,a1,24
     cfa:	00bb1cbb          	sllw	s9,s6,a1
     cfe:	00bb9d3b          	sllw	s10,s7,a1
     d02:	a02d                	j	d2c <.LBB4_21+0x4c>
     d04:	00150613          	addi	a2,a0,1
     d08:	6c14                	ld	a3,24(s0)
     d0a:	8df1                	and	a1,a1,a2
     d0c:	e80c                	sd	a1,16(s0)
     d0e:	050e                	slli	a0,a0,0x3
     d10:	9536                	add	a0,a0,a3
     d12:	01b53023          	sd	s11,0(a0)
     d16:	0310000f          	fence	rw,w
     d1a:	00040023          	sb	zero,0(s0)
     d1e:	8522                	mv	a0,s0
     d20:	00000097          	auipc	ra,0x0
     d24:	e0a080e7          	jalr	-502(ra) # b2a <_ZN12tornado_user7excutor17Executor$LT$T$GT$8pop_task17h0b9a58e7f13782dcE>
     d28:	12050a63          	beqz	a0,e5c <.LBB4_21+0x17c>
     d2c:	e42a                	sd	a0,8(sp)
     d2e:	02850513          	addi	a0,a0,40
     d32:	ffc57593          	andi	a1,a0,-4
     d36:	00357613          	andi	a2,a0,3
     d3a:	060e                	slli	a2,a2,0x3
     d3c:	00cb16bb          	sllw	a3,s6,a2
     d40:	00cb963b          	sllw	a2,s7,a2
     d44:	1405a72f          	lr.w.aq	a4,(a1)
     d48:	00d777b3          	and	a5,a4,a3
     d4c:	eb81                	bnez	a5,d5c <.LBB4_21+0x7c>
     d4e:	00c747b3          	xor	a5,a4,a2
     d52:	8ff5                	and	a5,a5,a3
     d54:	8fb9                	xor	a5,a5,a4
     d56:	18f5a7af          	sc.w	a5,a5,(a1)
     d5a:	f7ed                	bnez	a5,d44 <.LBB4_21+0x64>
     d5c:	00d77633          	and	a2,a4,a3
     d60:	1602                	slli	a2,a2,0x20
     d62:	9201                	srli	a2,a2,0x20
     d64:	ca11                	beqz	a2,d78 <.LBB4_21+0x98>
     d66:	00050603          	lb	a2,0(a0)
     d6a:	0ff67613          	andi	a2,a2,255
     d6e:	fe65                	bnez	a2,d66 <.LBB4_21+0x86>
     d70:	00351613          	slli	a2,a0,0x3
     d74:	8a61                	andi	a2,a2,24
     d76:	b7d9                	j	d3c <.LBB4_21+0x5c>
     d78:	017500a3          	sb	s7,1(a0)
     d7c:	0310000f          	fence	rw,w
     d80:	00050023          	sb	zero,0(a0)
     d84:	6da2                	ld	s11,8(sp)
     d86:	010d8493          	addi	s1,s11,16
     d8a:	8526                	mv	a0,s1
     d8c:	85d6                	mv	a1,s5
     d8e:	00001097          	auipc	ra,0x1
     d92:	0a2080e7          	jalr	162(ra) # 1e30 <_ZN4woke8WakerRef11new_unowned17h502a062306ae1e43E>
     d96:	e82a                	sd	a0,16(sp)
     d98:	ec2e                	sd	a1,24(sp)
     d9a:	854e                	mv	a0,s3
     d9c:	00001097          	auipc	ra,0x1
     da0:	096080e7          	jalr	150(ra) # 1e32 <_ZN58_$LT$woke..WakerRef$u20$as$u20$core..ops..deref..Deref$GT$5deref17h2fd191855ca923cdE>
     da4:	f02a                	sd	a0,32(sp)
     da6:	ffc4f513          	andi	a0,s1,-4
     daa:	0034f593          	andi	a1,s1,3
     dae:	058e                	slli	a1,a1,0x3
     db0:	00bb163b          	sllw	a2,s6,a1
     db4:	00bb95bb          	sllw	a1,s7,a1
     db8:	140526af          	lr.w.aq	a3,(a0)
     dbc:	00c6f733          	and	a4,a3,a2
     dc0:	eb01                	bnez	a4,dd0 <.LBB4_21+0xf0>
     dc2:	00b6c733          	xor	a4,a3,a1
     dc6:	8f71                	and	a4,a4,a2
     dc8:	8f35                	xor	a4,a4,a3
     dca:	18e5272f          	sc.w	a4,a4,(a0)
     dce:	f76d                	bnez	a4,db8 <.LBB4_21+0xd8>
     dd0:	00c6f5b3          	and	a1,a3,a2
     dd4:	1582                	slli	a1,a1,0x20
     dd6:	9181                	srli	a1,a1,0x20
     dd8:	c991                	beqz	a1,dec <.LBB4_21+0x10c>
     dda:	00048583          	lb	a1,0(s1)
     dde:	0ff5f593          	andi	a1,a1,255
     de2:	fde5                	bnez	a1,dda <.LBB4_21+0xfa>
     de4:	00349593          	slli	a1,s1,0x3
     de8:	89e1                	andi	a1,a1,24
     dea:	b7d9                	j	db0 <.LBB4_21+0xd0>
     dec:	020db583          	ld	a1,32(s11)
     df0:	018db503          	ld	a0,24(s11)
     df4:	6d90                	ld	a2,24(a1)
     df6:	85d2                	mv	a1,s4
     df8:	9602                	jalr	a2
     dfa:	0310000f          	fence	rw,w
     dfe:	000d8823          	sb	zero,16(s11)
     e02:	cd39                	beqz	a0,e60 <.LBB4_21+0x180>
     e04:	140c252f          	lr.w.aq	a0,(s8)
     e08:	019575b3          	and	a1,a0,s9
     e0c:	e989                	bnez	a1,e1e <.LBB4_21+0x13e>
     e0e:	01a545b3          	xor	a1,a0,s10
     e12:	0195f5b3          	and	a1,a1,s9
     e16:	8da9                	xor	a1,a1,a0
     e18:	18bc25af          	sc.w	a1,a1,(s8)
     e1c:	f5e5                	bnez	a1,e04 <.LBB4_21+0x124>
     e1e:	01957533          	and	a0,a0,s9
     e22:	1502                	slli	a0,a0,0x20
     e24:	9101                	srli	a0,a0,0x20
     e26:	c519                	beqz	a0,e34 <.LBB4_21+0x154>
     e28:	00040503          	lb	a0,0(s0)
     e2c:	0ff57513          	andi	a0,a0,255
     e30:	fd65                	bnez	a0,e28 <.LBB4_21+0x148>
     e32:	bfc9                	j	e04 <.LBB4_21+0x124>
     e34:	640c                	ld	a1,8(s0)
     e36:	6808                	ld	a0,16(s0)
     e38:	7010                	ld	a2,32(s0)
     e3a:	40b506b3          	sub	a3,a0,a1
     e3e:	fff60593          	addi	a1,a2,-1
     e42:	8eed                	and	a3,a3,a1
     e44:	8e15                	sub	a2,a2,a3
     e46:	eb761fe3          	bne	a2,s7,d04 <.LBB4_21+0x24>
     e4a:	854a                	mv	a0,s2
     e4c:	00000097          	auipc	ra,0x0
     e50:	430080e7          	jalr	1072(ra) # 127c <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17hdc31b30d85414892E.llvm.227590070318396806>
     e54:	700c                	ld	a1,32(s0)
     e56:	6808                	ld	a0,16(s0)
     e58:	15fd                	addi	a1,a1,-1
     e5a:	b56d                	j	d04 <.LBB4_21+0x24>
     e5c:	4401                	li	s0,0
     e5e:	a005                	j	e7e <.LBB4_21+0x19e>
     e60:	84ae                	mv	s1,a1
     e62:	4405                	li	s0,1
     e64:	40800533          	neg	a0,s0
     e68:	02adb52f          	amoadd.d.rl	a0,a0,(s11)
     e6c:	00851963          	bne	a0,s0,e7e <.LBB4_21+0x19e>
     e70:	0230000f          	fence	r,rw
     e74:	0028                	addi	a0,sp,8
     e76:	00000097          	auipc	ra,0x0
     e7a:	1b4080e7          	jalr	436(ra) # 102a <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17hab5dc6b9af68aca8E>
     e7e:	8522                	mv	a0,s0
     e80:	85a6                	mv	a1,s1
     e82:	7da2                	ld	s11,40(sp)
     e84:	7d42                	ld	s10,48(sp)
     e86:	7ce2                	ld	s9,56(sp)
     e88:	6c06                	ld	s8,64(sp)
     e8a:	6ba6                	ld	s7,72(sp)
     e8c:	6b46                	ld	s6,80(sp)
     e8e:	6ae6                	ld	s5,88(sp)
     e90:	7a06                	ld	s4,96(sp)
     e92:	79a6                	ld	s3,104(sp)
     e94:	7946                	ld	s2,112(sp)
     e96:	74e6                	ld	s1,120(sp)
     e98:	640a                	ld	s0,128(sp)
     e9a:	60aa                	ld	ra,136(sp)
     e9c:	6149                	addi	sp,sp,144
     e9e:	8082                	ret

0000000000000ea0 <_ZN75_$LT$tornado_user..excutor..EXECUTOR$u20$as$u20$core..ops..deref..Deref$GT$5deref17hee4014ce0b3f475dE>:
     ea0:	00006517          	auipc	a0,0x6
     ea4:	16050513          	addi	a0,a0,352 # 7000 <_ZN75_$LT$tornado_user..excutor..EXECUTOR$u20$as$u20$core..ops..deref..Deref$GT$5deref11__stability4LAZY17h53ae8f4cd44a05c3E>
     ea8:	00000317          	auipc	t1,0x0
     eac:	7c630067          	jr	1990(t1) # 166e <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE>

0000000000000eb0 <_ZN4woke12drop_arc_raw17hbe6ac949c5d1e29fE.llvm.2125609300770059803>:
     eb0:	1141                	addi	sp,sp,-16
     eb2:	e406                	sd	ra,8(sp)
     eb4:	1541                	addi	a0,a0,-16
     eb6:	e02a                	sd	a0,0(sp)
     eb8:	4585                	li	a1,1
     eba:	40b00633          	neg	a2,a1
     ebe:	02c5352f          	amoadd.d.rl	a0,a2,(a0)
     ec2:	00b51963          	bne	a0,a1,ed4 <_ZN4woke12drop_arc_raw17hbe6ac949c5d1e29fE.llvm.2125609300770059803+0x24>
     ec6:	0230000f          	fence	r,rw
     eca:	850a                	mv	a0,sp
     ecc:	00000097          	auipc	ra,0x0
     ed0:	15e080e7          	jalr	350(ra) # 102a <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17hab5dc6b9af68aca8E>
     ed4:	60a2                	ld	ra,8(sp)
     ed6:	0141                	addi	sp,sp,16
     ed8:	8082                	ret

0000000000000eda <_ZN4woke12wake_arc_raw17h6ae4e5508a761a0eE.llvm.2125609300770059803>:
     eda:	1141                	addi	sp,sp,-16
     edc:	e406                	sd	ra,8(sp)
     ede:	ff050593          	addi	a1,a0,-16
     ee2:	e02e                	sd	a1,0(sp)
     ee4:	0561                	addi	a0,a0,24
     ee6:	ffc57613          	andi	a2,a0,-4
     eea:	00357593          	andi	a1,a0,3
     eee:	00359713          	slli	a4,a1,0x3
     ef2:	0ff00813          	li	a6,255
     ef6:	00e817bb          	sllw	a5,a6,a4
     efa:	4885                	li	a7,1
     efc:	00e8973b          	sllw	a4,a7,a4
     f00:	140626af          	lr.w.aq	a3,(a2)
     f04:	00f6f5b3          	and	a1,a3,a5
     f08:	e981                	bnez	a1,f18 <_ZN4woke12wake_arc_raw17h6ae4e5508a761a0eE.llvm.2125609300770059803+0x3e>
     f0a:	00e6c5b3          	xor	a1,a3,a4
     f0e:	8dfd                	and	a1,a1,a5
     f10:	8db5                	xor	a1,a1,a3
     f12:	18b625af          	sc.w	a1,a1,(a2)
     f16:	f5ed                	bnez	a1,f00 <_ZN4woke12wake_arc_raw17h6ae4e5508a761a0eE.llvm.2125609300770059803+0x26>
     f18:	00f6f5b3          	and	a1,a3,a5
     f1c:	1582                	slli	a1,a1,0x20
     f1e:	9181                	srli	a1,a1,0x20
     f20:	cd8d                	beqz	a1,f5a <_ZN4woke12wake_arc_raw17h6ae4e5508a761a0eE.llvm.2125609300770059803+0x80>
     f22:	00351593          	slli	a1,a0,0x3
     f26:	89e1                	andi	a1,a1,24
     f28:	00b816bb          	sllw	a3,a6,a1
     f2c:	00b8973b          	sllw	a4,a7,a1
     f30:	00050583          	lb	a1,0(a0)
     f34:	0ff5f593          	andi	a1,a1,255
     f38:	fde5                	bnez	a1,f30 <_ZN4woke12wake_arc_raw17h6ae4e5508a761a0eE.llvm.2125609300770059803+0x56>
     f3a:	140625af          	lr.w.aq	a1,(a2)
     f3e:	00d5f7b3          	and	a5,a1,a3
     f42:	eb81                	bnez	a5,f52 <_ZN4woke12wake_arc_raw17h6ae4e5508a761a0eE.llvm.2125609300770059803+0x78>
     f44:	00e5c7b3          	xor	a5,a1,a4
     f48:	8ff5                	and	a5,a5,a3
     f4a:	8fad                	xor	a5,a5,a1
     f4c:	18f627af          	sc.w	a5,a5,(a2)
     f50:	f7ed                	bnez	a5,f3a <_ZN4woke12wake_arc_raw17h6ae4e5508a761a0eE.llvm.2125609300770059803+0x60>
     f52:	8df5                	and	a1,a1,a3
     f54:	1582                	slli	a1,a1,0x20
     f56:	9181                	srli	a1,a1,0x20
     f58:	fde1                	bnez	a1,f30 <_ZN4woke12wake_arc_raw17h6ae4e5508a761a0eE.llvm.2125609300770059803+0x56>
     f5a:	000500a3          	sb	zero,1(a0)
     f5e:	0310000f          	fence	rw,w
     f62:	00050023          	sb	zero,0(a0)
     f66:	6502                	ld	a0,0(sp)
     f68:	411005b3          	neg	a1,a7
     f6c:	02b5352f          	amoadd.d.rl	a0,a1,(a0)
     f70:	01151963          	bne	a0,a7,f82 <_ZN4woke12wake_arc_raw17h6ae4e5508a761a0eE.llvm.2125609300770059803+0xa8>
     f74:	0230000f          	fence	r,rw
     f78:	850a                	mv	a0,sp
     f7a:	00000097          	auipc	ra,0x0
     f7e:	0b0080e7          	jalr	176(ra) # 102a <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17hab5dc6b9af68aca8E>
     f82:	60a2                	ld	ra,8(sp)
     f84:	0141                	addi	sp,sp,16
     f86:	8082                	ret

0000000000000f88 <_ZN4woke13clone_arc_raw17h6ca55915d7b3b8deE.llvm.2125609300770059803>:
     f88:	ff050593          	addi	a1,a0,-16
     f8c:	4605                	li	a2,1
     f8e:	00c5b5af          	amoadd.d	a1,a2,(a1)
     f92:	567d                	li	a2,-1
     f94:	00b65763          	bge	a2,a1,fa2 <.LBB3_3+0xa>

0000000000000f98 <.LBB3_3>:
     f98:	00004597          	auipc	a1,0x4
     f9c:	34058593          	addi	a1,a1,832 # 52d8 <_ZN12tornado_user7excutor8EXECUTOR17hf6915d4efcbd1205E>
     fa0:	8082                	ret
     fa2:	0000                	unimp
	...

0000000000000fa6 <_ZN4woke19wake_by_ref_arc_raw17h67726b2b25e090dfE.llvm.2125609300770059803>:
     fa6:	0561                	addi	a0,a0,24
     fa8:	ffc57593          	andi	a1,a0,-4
     fac:	00357613          	andi	a2,a0,3
     fb0:	00361713          	slli	a4,a2,0x3
     fb4:	0ff00813          	li	a6,255
     fb8:	00e817bb          	sllw	a5,a6,a4
     fbc:	4885                	li	a7,1
     fbe:	00e8973b          	sllw	a4,a7,a4
     fc2:	1405a62f          	lr.w.aq	a2,(a1)
     fc6:	00f676b3          	and	a3,a2,a5
     fca:	ea81                	bnez	a3,fda <_ZN4woke19wake_by_ref_arc_raw17h67726b2b25e090dfE.llvm.2125609300770059803+0x34>
     fcc:	00e646b3          	xor	a3,a2,a4
     fd0:	8efd                	and	a3,a3,a5
     fd2:	8eb1                	xor	a3,a3,a2
     fd4:	18d5a6af          	sc.w	a3,a3,(a1)
     fd8:	f6ed                	bnez	a3,fc2 <_ZN4woke19wake_by_ref_arc_raw17h67726b2b25e090dfE.llvm.2125609300770059803+0x1c>
     fda:	8e7d                	and	a2,a2,a5
     fdc:	1602                	slli	a2,a2,0x20
     fde:	9201                	srli	a2,a2,0x20
     fe0:	ce15                	beqz	a2,101c <_ZN4woke19wake_by_ref_arc_raw17h67726b2b25e090dfE.llvm.2125609300770059803+0x76>
     fe2:	00351613          	slli	a2,a0,0x3
     fe6:	01867693          	andi	a3,a2,24
     fea:	00d8163b          	sllw	a2,a6,a3
     fee:	00d896bb          	sllw	a3,a7,a3
     ff2:	00050703          	lb	a4,0(a0)
     ff6:	0ff77713          	andi	a4,a4,255
     ffa:	ff65                	bnez	a4,ff2 <_ZN4woke19wake_by_ref_arc_raw17h67726b2b25e090dfE.llvm.2125609300770059803+0x4c>
     ffc:	1405a72f          	lr.w.aq	a4,(a1)
    1000:	00c777b3          	and	a5,a4,a2
    1004:	eb81                	bnez	a5,1014 <_ZN4woke19wake_by_ref_arc_raw17h67726b2b25e090dfE.llvm.2125609300770059803+0x6e>
    1006:	00d747b3          	xor	a5,a4,a3
    100a:	8ff1                	and	a5,a5,a2
    100c:	8fb9                	xor	a5,a5,a4
    100e:	18f5a7af          	sc.w	a5,a5,(a1)
    1012:	f7ed                	bnez	a5,ffc <_ZN4woke19wake_by_ref_arc_raw17h67726b2b25e090dfE.llvm.2125609300770059803+0x56>
    1014:	8f71                	and	a4,a4,a2
    1016:	1702                	slli	a4,a4,0x20
    1018:	9301                	srli	a4,a4,0x20
    101a:	ff61                	bnez	a4,ff2 <_ZN4woke19wake_by_ref_arc_raw17h67726b2b25e090dfE.llvm.2125609300770059803+0x4c>
    101c:	000500a3          	sb	zero,1(a0)
    1020:	0310000f          	fence	rw,w
    1024:	00050023          	sb	zero,0(a0)
    1028:	8082                	ret

000000000000102a <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17hab5dc6b9af68aca8E>:
    102a:	1101                	addi	sp,sp,-32
    102c:	ec06                	sd	ra,24(sp)
    102e:	e822                	sd	s0,16(sp)
    1030:	e426                	sd	s1,8(sp)
    1032:	842a                	mv	s0,a0
    1034:	6104                	ld	s1,0(a0)
    1036:	708c                	ld	a1,32(s1)
    1038:	6c88                	ld	a0,24(s1)
    103a:	618c                	ld	a1,0(a1)
    103c:	9582                	jalr	a1
    103e:	7090                	ld	a2,32(s1)
    1040:	660c                	ld	a1,8(a2)
    1042:	c599                	beqz	a1,1050 <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17hab5dc6b9af68aca8E+0x26>
    1044:	6c88                	ld	a0,24(s1)
    1046:	6a10                	ld	a2,16(a2)
    1048:	00000097          	auipc	ra,0x0
    104c:	ac2080e7          	jalr	-1342(ra) # b0a <__rust_dealloc>
    1050:	6008                	ld	a0,0(s0)
    1052:	55fd                	li	a1,-1
    1054:	02b50863          	beq	a0,a1,1084 <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17hab5dc6b9af68aca8E+0x5a>
    1058:	00850593          	addi	a1,a0,8
    105c:	4605                	li	a2,1
    105e:	40c006b3          	neg	a3,a2
    1062:	02d5b5af          	amoadd.d.rl	a1,a3,(a1)
    1066:	00c59f63          	bne	a1,a2,1084 <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17hab5dc6b9af68aca8E+0x5a>
    106a:	03000593          	li	a1,48
    106e:	4621                	li	a2,8
    1070:	0230000f          	fence	r,rw
    1074:	64a2                	ld	s1,8(sp)
    1076:	6442                	ld	s0,16(sp)
    1078:	60e2                	ld	ra,24(sp)
    107a:	6105                	addi	sp,sp,32
    107c:	00000317          	auipc	t1,0x0
    1080:	a8e30067          	jr	-1394(t1) # b0a <__rust_dealloc>
    1084:	64a2                	ld	s1,8(sp)
    1086:	6442                	ld	s0,16(sp)
    1088:	60e2                	ld	ra,24(sp)
    108a:	6105                	addi	sp,sp,32
    108c:	8082                	ret

000000000000108e <_ZN12tornado_user4task10UserTaskId8generate17hb5138570cf78f129E>:
    108e:	0000e517          	auipc	a0,0xe
    1092:	0d250513          	addi	a0,a0,210 # f160 <_ZN12tornado_user4task10UserTaskId8generate7COUNTER17h13230ff31d38f7aaE>
    1096:	4585                	li	a1,1
    1098:	00b5352f          	amoadd.d	a0,a1,(a0)
    109c:	55fd                	li	a1,-1
    109e:	00a5d363          	bge	a1,a0,10a4 <.LBB1_4>
    10a2:	8082                	ret

00000000000010a4 <.LBB1_4>:
    10a4:	00004517          	auipc	a0,0x4
    10a8:	35c50513          	addi	a0,a0,860 # 5400 <.Lanon.a6cd8df4e24dac0f0dc7ecbcbd5a7bc0.0>

00000000000010ac <.LBB1_5>:
    10ac:	00004617          	auipc	a2,0x4
    10b0:	37c60613          	addi	a2,a2,892 # 5428 <.Lanon.a6cd8df4e24dac0f0dc7ecbcbd5a7bc0.2>
    10b4:	45bd                	li	a1,15
    10b6:	00001097          	auipc	ra,0x1
    10ba:	e8c080e7          	jalr	-372(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
	...

00000000000010c0 <_ZN12tornado_user4task8UserTask18shared_task_handle17h946cb8771506c97bE>:
    10c0:	03059603          	lh	a2,48(a1)
    10c4:	05c1                	addi	a1,a1,16
    10c6:	00053023          	sd	zero,0(a0)
    10ca:	00c51423          	sh	a2,8(a0)
    10ce:	e90c                	sd	a1,16(a0)
    10d0:	8082                	ret

00000000000010d2 <_ZN12tornado_user4task8UserTask11is_sleeping17h921cc2599b53a729E>:
    10d2:	02250593          	addi	a1,a0,34
    10d6:	ffc5f313          	andi	t1,a1,-4
    10da:	0035f693          	andi	a3,a1,3
    10de:	00369793          	slli	a5,a3,0x3
    10e2:	0ff00813          	li	a6,255
    10e6:	00f812bb          	sllw	t0,a6,a5
    10ea:	4885                	li	a7,1
    10ec:	00f897bb          	sllw	a5,a7,a5
    10f0:	1403272f          	lr.w.aq	a4,(t1)
    10f4:	005776b3          	and	a3,a4,t0
    10f8:	ea89                	bnez	a3,110a <_ZN12tornado_user4task8UserTask11is_sleeping17h921cc2599b53a729E+0x38>
    10fa:	00f746b3          	xor	a3,a4,a5
    10fe:	0056f6b3          	and	a3,a3,t0
    1102:	8eb9                	xor	a3,a3,a4
    1104:	18d326af          	sc.w	a3,a3,(t1)
    1108:	f6e5                	bnez	a3,10f0 <_ZN12tornado_user4task8UserTask11is_sleeping17h921cc2599b53a729E+0x1e>
    110a:	005776b3          	and	a3,a4,t0
    110e:	1682                	slli	a3,a3,0x20
    1110:	9281                	srli	a3,a3,0x20
    1112:	ce9d                	beqz	a3,1150 <_ZN12tornado_user4task8UserTask11is_sleeping17h921cc2599b53a729E+0x7e>
    1114:	00359693          	slli	a3,a1,0x3
    1118:	0186f713          	andi	a4,a3,24
    111c:	00e816bb          	sllw	a3,a6,a4
    1120:	00e8973b          	sllw	a4,a7,a4
    1124:	00058603          	lb	a2,0(a1)
    1128:	0ff67613          	andi	a2,a2,255
    112c:	fe65                	bnez	a2,1124 <_ZN12tornado_user4task8UserTask11is_sleeping17h921cc2599b53a729E+0x52>
    112e:	140327af          	lr.w.aq	a5,(t1)
    1132:	00d7f633          	and	a2,a5,a3
    1136:	ea01                	bnez	a2,1146 <_ZN12tornado_user4task8UserTask11is_sleeping17h921cc2599b53a729E+0x74>
    1138:	00e7c633          	xor	a2,a5,a4
    113c:	8e75                	and	a2,a2,a3
    113e:	8e3d                	xor	a2,a2,a5
    1140:	18c3262f          	sc.w	a2,a2,(t1)
    1144:	f66d                	bnez	a2,112e <_ZN12tornado_user4task8UserTask11is_sleeping17h921cc2599b53a729E+0x5c>
    1146:	00d7f633          	and	a2,a5,a3
    114a:	1602                	slli	a2,a2,0x20
    114c:	9201                	srli	a2,a2,0x20
    114e:	fa79                	bnez	a2,1124 <_ZN12tornado_user4task8UserTask11is_sleeping17h921cc2599b53a729E+0x52>
    1150:	02354583          	lbu	a1,35(a0)
    1154:	0310000f          	fence	rw,w
    1158:	02050123          	sb	zero,34(a0)
    115c:	852e                	mv	a0,a1
    115e:	8082                	ret

0000000000001160 <_ZN12tornado_user4task8UserTask10mark_sleep17he16c82000448e43fE>:
    1160:	02250593          	addi	a1,a0,34
    1164:	ffc5f313          	andi	t1,a1,-4
    1168:	0035f693          	andi	a3,a1,3
    116c:	00369793          	slli	a5,a3,0x3
    1170:	0ff00893          	li	a7,255
    1174:	00f892bb          	sllw	t0,a7,a5
    1178:	4805                	li	a6,1
    117a:	00f817bb          	sllw	a5,a6,a5
    117e:	1403272f          	lr.w.aq	a4,(t1)
    1182:	005776b3          	and	a3,a4,t0
    1186:	ea89                	bnez	a3,1198 <_ZN12tornado_user4task8UserTask10mark_sleep17he16c82000448e43fE+0x38>
    1188:	00f746b3          	xor	a3,a4,a5
    118c:	0056f6b3          	and	a3,a3,t0
    1190:	8eb9                	xor	a3,a3,a4
    1192:	18d326af          	sc.w	a3,a3,(t1)
    1196:	f6e5                	bnez	a3,117e <_ZN12tornado_user4task8UserTask10mark_sleep17he16c82000448e43fE+0x1e>
    1198:	005776b3          	and	a3,a4,t0
    119c:	1682                	slli	a3,a3,0x20
    119e:	9281                	srli	a3,a3,0x20
    11a0:	ce95                	beqz	a3,11dc <_ZN12tornado_user4task8UserTask10mark_sleep17he16c82000448e43fE+0x7c>
    11a2:	00359693          	slli	a3,a1,0x3
    11a6:	8ae1                	andi	a3,a3,24
    11a8:	00d8973b          	sllw	a4,a7,a3
    11ac:	00d817bb          	sllw	a5,a6,a3
    11b0:	00058603          	lb	a2,0(a1)
    11b4:	0ff67613          	andi	a2,a2,255
    11b8:	fe65                	bnez	a2,11b0 <_ZN12tornado_user4task8UserTask10mark_sleep17he16c82000448e43fE+0x50>
    11ba:	140326af          	lr.w.aq	a3,(t1)
    11be:	00e6f633          	and	a2,a3,a4
    11c2:	ea01                	bnez	a2,11d2 <_ZN12tornado_user4task8UserTask10mark_sleep17he16c82000448e43fE+0x72>
    11c4:	00f6c633          	xor	a2,a3,a5
    11c8:	8e79                	and	a2,a2,a4
    11ca:	8e35                	xor	a2,a2,a3
    11cc:	18c3262f          	sc.w	a2,a2,(t1)
    11d0:	f66d                	bnez	a2,11ba <_ZN12tornado_user4task8UserTask10mark_sleep17he16c82000448e43fE+0x5a>
    11d2:	00e6f633          	and	a2,a3,a4
    11d6:	1602                	slli	a2,a2,0x20
    11d8:	9201                	srli	a2,a2,0x20
    11da:	fa79                	bnez	a2,11b0 <_ZN12tornado_user4task8UserTask10mark_sleep17he16c82000448e43fE+0x50>
    11dc:	030501a3          	sb	a6,35(a0)
    11e0:	0310000f          	fence	rw,w
    11e4:	02050123          	sb	zero,34(a0)
    11e8:	8082                	ret

00000000000011ea <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE>:
    11ea:	00053283          	ld	t0,0(a0)
    11ee:	03228513          	addi	a0,t0,50
    11f2:	ffc57593          	andi	a1,a0,-4
    11f6:	00357693          	andi	a3,a0,3
    11fa:	00369793          	slli	a5,a3,0x3
    11fe:	0ff00813          	li	a6,255
    1202:	00f816bb          	sllw	a3,a6,a5
    1206:	4885                	li	a7,1
    1208:	00f897bb          	sllw	a5,a7,a5
    120c:	1405a72f          	lr.w.aq	a4,(a1)
    1210:	00d77633          	and	a2,a4,a3
    1214:	ea01                	bnez	a2,1224 <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE+0x3a>
    1216:	00f74633          	xor	a2,a4,a5
    121a:	8e75                	and	a2,a2,a3
    121c:	8e39                	xor	a2,a2,a4
    121e:	18c5a62f          	sc.w	a2,a2,(a1)
    1222:	f66d                	bnez	a2,120c <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE+0x22>
    1224:	00d77633          	and	a2,a4,a3
    1228:	1602                	slli	a2,a2,0x20
    122a:	02065693          	srli	a3,a2,0x20
    122e:	02c1                	addi	t0,t0,16
    1230:	ce9d                	beqz	a3,126e <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE+0x84>
    1232:	00351693          	slli	a3,a0,0x3
    1236:	0186f713          	andi	a4,a3,24
    123a:	00e816bb          	sllw	a3,a6,a4
    123e:	00e8973b          	sllw	a4,a7,a4
    1242:	00050603          	lb	a2,0(a0)
    1246:	0ff67613          	andi	a2,a2,255
    124a:	fe65                	bnez	a2,1242 <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE+0x58>
    124c:	1405a7af          	lr.w.aq	a5,(a1)
    1250:	00d7f633          	and	a2,a5,a3
    1254:	ea01                	bnez	a2,1264 <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE+0x7a>
    1256:	00e7c633          	xor	a2,a5,a4
    125a:	8e75                	and	a2,a2,a3
    125c:	8e3d                	xor	a2,a2,a5
    125e:	18c5a62f          	sc.w	a2,a2,(a1)
    1262:	f66d                	bnez	a2,124c <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE+0x62>
    1264:	00d7f633          	and	a2,a5,a3
    1268:	1602                	slli	a2,a2,0x20
    126a:	9201                	srli	a2,a2,0x20
    126c:	fa79                	bnez	a2,1242 <_ZN59_$LT$tornado_user..task..UserTask$u20$as$u20$woke..Woke$GT$11wake_by_ref17h9480cc90a6efd60bE+0x58>
    126e:	020281a3          	sb	zero,35(t0)
    1272:	0310000f          	fence	rw,w
    1276:	02028123          	sb	zero,34(t0)
    127a:	8082                	ret

000000000000127c <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17hdc31b30d85414892E.llvm.227590070318396806>:
    127c:	715d                	addi	sp,sp,-80
    127e:	e486                	sd	ra,72(sp)
    1280:	e0a2                	sd	s0,64(sp)
    1282:	fc26                	sd	s1,56(sp)
    1284:	f84a                	sd	s2,48(sp)
    1286:	842a                	mv	s0,a0
    1288:	6108                	ld	a0,0(a0)
    128a:	640c                	ld	a1,8(s0)
    128c:	6c04                	ld	s1,24(s0)
    128e:	40a58533          	sub	a0,a1,a0
    1292:	fff48593          	addi	a1,s1,-1
    1296:	8d6d                	and	a0,a0,a1
    1298:	40a48533          	sub	a0,s1,a0
    129c:	4585                	li	a1,1
    129e:	0cb51863          	bne	a0,a1,136e <.LBB2_18+0x38>
    12a2:	c0c9                	beqz	s1,1324 <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17hdc31b30d85414892E.llvm.227590070318396806+0xa8>
    12a4:	00948533          	add	a0,s1,s1
    12a8:	0c956a63          	bltu	a0,s1,137c <.LBB2_18+0x46>
    12ac:	4601                	li	a2,0
    12ae:	4905                	li	s2,1
    12b0:	03d91593          	slli	a1,s2,0x3d
    12b4:	15fd                	addi	a1,a1,-1
    12b6:	8de9                	and	a1,a1,a0
    12b8:	8da9                	xor	a1,a1,a0
    12ba:	00b036b3          	snez	a3,a1
    12be:	00351593          	slli	a1,a0,0x3
    12c2:	4521                	li	a0,8
    12c4:	e291                	bnez	a3,12c8 <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17hdc31b30d85414892E.llvm.227590070318396806+0x4c>
    12c6:	4621                	li	a2,8
    12c8:	6814                	ld	a3,16(s0)
    12ca:	00349713          	slli	a4,s1,0x3
    12ce:	ec36                	sd	a3,24(sp)
    12d0:	f03a                	sd	a4,32(sp)
    12d2:	f42a                	sd	a0,40(sp)
    12d4:	850a                	mv	a0,sp
    12d6:	0834                	addi	a3,sp,24
    12d8:	00000097          	auipc	ra,0x0
    12dc:	19a080e7          	jalr	410(ra) # 1472 <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370>
    12e0:	6602                	ld	a2,0(sp)
    12e2:	6522                	ld	a0,8(sp)
    12e4:	65c2                	ld	a1,16(sp)
    12e6:	09260a63          	beq	a2,s2,137a <.LBB2_18+0x44>
    12ea:	e808                	sd	a0,16(s0)
    12ec:	0035d513          	srli	a0,a1,0x3
    12f0:	ec08                	sd	a0,24(s0)
    12f2:	00149593          	slli	a1,s1,0x1
    12f6:	02b51c63          	bne	a0,a1,132e <.LBB2_17>
    12fa:	600c                	ld	a1,0(s0)
    12fc:	6410                	ld	a2,8(s0)
    12fe:	06b67863          	bgeu	a2,a1,136e <.LBB2_18+0x38>
    1302:	40b486b3          	sub	a3,s1,a1
    1306:	04d67363          	bgeu	a2,a3,134c <.LBB2_18+0x16>
    130a:	680c                	ld	a1,16(s0)
    130c:	00349513          	slli	a0,s1,0x3
    1310:	952e                	add	a0,a0,a1
    1312:	060e                	slli	a2,a2,0x3
    1314:	00003097          	auipc	ra,0x3
    1318:	ce4080e7          	jalr	-796(ra) # 3ff8 <memcpy>
    131c:	6408                	ld	a0,8(s0)
    131e:	9526                	add	a0,a0,s1
    1320:	e408                	sd	a0,8(s0)
    1322:	a0b1                	j	136e <.LBB2_18+0x38>
    1324:	4501                	li	a0,0
    1326:	00149593          	slli	a1,s1,0x1
    132a:	fcb508e3          	beq	a0,a1,12fa <_ZN5alloc11collections9vec_deque17VecDeque$LT$T$GT$4grow17hdc31b30d85414892E.llvm.227590070318396806+0x7e>

000000000000132e <.LBB2_17>:
    132e:	00004517          	auipc	a0,0x4
    1332:	17050513          	addi	a0,a0,368 # 549e <.Lanon.24b97dc0e08433fc2bb4290f4e5e47d5.1>

0000000000001336 <.LBB2_18>:
    1336:	00004617          	auipc	a2,0x4
    133a:	19a60613          	addi	a2,a2,410 # 54d0 <.Lanon.24b97dc0e08433fc2bb4290f4e5e47d5.2>
    133e:	02b00593          	li	a1,43
    1342:	00001097          	auipc	ra,0x1
    1346:	c00080e7          	jalr	-1024(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
    134a:	0000                	unimp
    134c:	6810                	ld	a2,16(s0)
    134e:	40d504b3          	sub	s1,a0,a3
    1352:	00359513          	slli	a0,a1,0x3
    1356:	00a605b3          	add	a1,a2,a0
    135a:	00349513          	slli	a0,s1,0x3
    135e:	9532                	add	a0,a0,a2
    1360:	00369613          	slli	a2,a3,0x3
    1364:	00003097          	auipc	ra,0x3
    1368:	c94080e7          	jalr	-876(ra) # 3ff8 <memcpy>
    136c:	e004                	sd	s1,0(s0)
    136e:	7942                	ld	s2,48(sp)
    1370:	74e2                	ld	s1,56(sp)
    1372:	6406                	ld	s0,64(sp)
    1374:	60a6                	ld	ra,72(sp)
    1376:	6161                	addi	sp,sp,80
    1378:	8082                	ret
    137a:	e591                	bnez	a1,1386 <.LBB2_18+0x50>
    137c:	00001097          	auipc	ra,0x1
    1380:	ae6080e7          	jalr	-1306(ra) # 1e62 <_ZN5alloc7raw_vec17capacity_overflow17hb114381a505af03eE>
    1384:	0000                	unimp
    1386:	00001097          	auipc	ra,0x1
    138a:	ac0080e7          	jalr	-1344(ra) # 1e46 <_ZN5alloc5alloc18handle_alloc_error17h0809b9ba7eebe66bE>
	...

0000000000001390 <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E>:
    1390:	7139                	addi	sp,sp,-64
    1392:	fc06                	sd	ra,56(sp)
    1394:	f822                	sd	s0,48(sp)
    1396:	f426                	sd	s1,40(sp)
    1398:	f04a                	sd	s2,32(sp)
    139a:	ec4e                	sd	s3,24(sp)
    139c:	e852                	sd	s4,16(sp)
    139e:	e456                	sd	s5,8(sp)
    13a0:	85aa                	mv	a1,a0
    13a2:	6508                	ld	a0,8(a0)
    13a4:	6190                	ld	a2,0(a1)
    13a6:	0105b983          	ld	s3,16(a1)
    13aa:	6d8c                	ld	a1,24(a1)
    13ac:	00c57863          	bgeu	a0,a2,13bc <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x2c>
    13b0:	892a                	mv	s2,a0
    13b2:	08c5e863          	bltu	a1,a2,1442 <.LBB5_17>
    13b6:	00c58963          	beq	a1,a2,13c8 <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x38>
    13ba:	a889                	j	140c <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x7c>
    13bc:	0aa5e263          	bltu	a1,a0,1460 <.LBB5_19>
    13c0:	4901                	li	s2,0
    13c2:	85aa                	mv	a1,a0
    13c4:	04c59463          	bne	a1,a2,140c <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x7c>
    13c8:	02090963          	beqz	s2,13fa <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x6a>
    13cc:	00391413          	slli	s0,s2,0x3
    13d0:	4905                	li	s2,1
    13d2:	412004b3          	neg	s1,s2
    13d6:	a021                	j	13de <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x4e>
    13d8:	1461                	addi	s0,s0,-8
    13da:	09a1                	addi	s3,s3,8
    13dc:	cc19                	beqz	s0,13fa <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x6a>
    13de:	0009b503          	ld	a0,0(s3)
    13e2:	0295352f          	amoadd.d.rl	a0,s1,(a0)
    13e6:	ff2519e3          	bne	a0,s2,13d8 <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x48>
    13ea:	0230000f          	fence	r,rw
    13ee:	854e                	mv	a0,s3
    13f0:	00000097          	auipc	ra,0x0
    13f4:	c3a080e7          	jalr	-966(ra) # 102a <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17hab5dc6b9af68aca8E>
    13f8:	b7c5                	j	13d8 <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x48>
    13fa:	6aa2                	ld	s5,8(sp)
    13fc:	6a42                	ld	s4,16(sp)
    13fe:	69e2                	ld	s3,24(sp)
    1400:	7902                	ld	s2,32(sp)
    1402:	74a2                	ld	s1,40(sp)
    1404:	7442                	ld	s0,48(sp)
    1406:	70e2                	ld	ra,56(sp)
    1408:	6121                	addi	sp,sp,64
    140a:	8082                	ret
    140c:	00361513          	slli	a0,a2,0x3
    1410:	00a984b3          	add	s1,s3,a0
    1414:	058e                	slli	a1,a1,0x3
    1416:	40a58433          	sub	s0,a1,a0
    141a:	4a05                	li	s4,1
    141c:	41400ab3          	neg	s5,s4
    1420:	a021                	j	1428 <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x98>
    1422:	1461                	addi	s0,s0,-8
    1424:	04a1                	addi	s1,s1,8
    1426:	d04d                	beqz	s0,13c8 <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x38>
    1428:	6088                	ld	a0,0(s1)
    142a:	0355352f          	amoadd.d.rl	a0,s5,(a0)
    142e:	ff451ae3          	bne	a0,s4,1422 <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x92>
    1432:	0230000f          	fence	r,rw
    1436:	8526                	mv	a0,s1
    1438:	00000097          	auipc	ra,0x0
    143c:	bf2080e7          	jalr	-1038(ra) # 102a <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17hab5dc6b9af68aca8E>
    1440:	b7cd                	j	1422 <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E+0x92>

0000000000001442 <.LBB5_17>:
    1442:	00004517          	auipc	a0,0x4
    1446:	eb650513          	addi	a0,a0,-330 # 52f8 <anon.94a30639c8e43a3f584e10ab8b53be30.0.llvm.1569364017202141003>

000000000000144a <.LBB5_18>:
    144a:	00004617          	auipc	a2,0x4
    144e:	f1e60613          	addi	a2,a2,-226 # 5368 <anon.94a30639c8e43a3f584e10ab8b53be30.2.llvm.1569364017202141003>
    1452:	02300593          	li	a1,35
    1456:	00001097          	auipc	ra,0x1
    145a:	aec080e7          	jalr	-1300(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
	...

0000000000001460 <.LBB5_19>:
    1460:	00004617          	auipc	a2,0x4
    1464:	f8860613          	addi	a2,a2,-120 # 53e8 <anon.94a30639c8e43a3f584e10ab8b53be30.4.llvm.1569364017202141003>
    1468:	00002097          	auipc	ra,0x2
    146c:	d9c080e7          	jalr	-612(ra) # 3204 <_ZN4core5slice5index24slice_end_index_len_fail17h0db61fbd8d9e0e45E>
	...

0000000000001472 <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370>:
    1472:	1101                	addi	sp,sp,-32
    1474:	ec06                	sd	ra,24(sp)
    1476:	e822                	sd	s0,16(sp)
    1478:	e426                	sd	s1,8(sp)
    147a:	e04a                	sd	s2,0(sp)
    147c:	84ae                	mv	s1,a1
    147e:	842a                	mv	s0,a0
    1480:	ce11                	beqz	a2,149c <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370+0x2a>
    1482:	8932                	mv	s2,a2
    1484:	6288                	ld	a0,0(a3)
    1486:	cd19                	beqz	a0,14a4 <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370+0x32>
    1488:	668c                	ld	a1,8(a3)
    148a:	cd89                	beqz	a1,14a4 <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370+0x32>
    148c:	864a                	mv	a2,s2
    148e:	86a6                	mv	a3,s1
    1490:	fffff097          	auipc	ra,0xfffff
    1494:	682080e7          	jalr	1666(ra) # b12 <__rust_realloc>
    1498:	e11d                	bnez	a0,14be <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370+0x4c>
    149a:	a829                	j	14b4 <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370+0x42>
    149c:	e404                	sd	s1,8(s0)
    149e:	4585                	li	a1,1
    14a0:	4481                	li	s1,0
    14a2:	a005                	j	14c2 <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370+0x50>
    14a4:	cc81                	beqz	s1,14bc <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370+0x4a>
    14a6:	8526                	mv	a0,s1
    14a8:	85ca                	mv	a1,s2
    14aa:	fffff097          	auipc	ra,0xfffff
    14ae:	658080e7          	jalr	1624(ra) # b02 <__rust_alloc>
    14b2:	e511                	bnez	a0,14be <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370+0x4c>
    14b4:	e404                	sd	s1,8(s0)
    14b6:	4585                	li	a1,1
    14b8:	84ca                	mv	s1,s2
    14ba:	a021                	j	14c2 <_ZN5alloc7raw_vec11finish_grow17h59e10aa13600d24fE.llvm.14527222005558606370+0x50>
    14bc:	854a                	mv	a0,s2
    14be:	4581                	li	a1,0
    14c0:	e408                	sd	a0,8(s0)
    14c2:	e804                	sd	s1,16(s0)
    14c4:	e00c                	sd	a1,0(s0)
    14c6:	6902                	ld	s2,0(sp)
    14c8:	64a2                	ld	s1,8(sp)
    14ca:	6442                	ld	s0,16(sp)
    14cc:	60e2                	ld	ra,24(sp)
    14ce:	6105                	addi	sp,sp,32
    14d0:	8082                	ret

00000000000014d2 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hbf98704c9ff0dbaaE>:
    14d2:	1141                	addi	sp,sp,-16
    14d4:	e406                	sd	ra,8(sp)
    14d6:	e022                	sd	s0,0(sp)
    14d8:	4605                	li	a2,1
    14da:	1676                	slli	a2,a2,0x3d
    14dc:	167d                	addi	a2,a2,-1
    14de:	8e69                	and	a2,a2,a0
    14e0:	04a61663          	bne	a2,a0,152c <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hbf98704c9ff0dbaaE+0x5a>
    14e4:	00351413          	slli	s0,a0,0x3
    14e8:	00143513          	seqz	a0,s0
    14ec:	c991                	beqz	a1,1500 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hbf98704c9ff0dbaaE+0x2e>
    14ee:	e905                	bnez	a0,151e <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hbf98704c9ff0dbaaE+0x4c>
    14f0:	45a1                	li	a1,8
    14f2:	8522                	mv	a0,s0
    14f4:	fffff097          	auipc	ra,0xfffff
    14f8:	626080e7          	jalr	1574(ra) # b1a <__rust_alloc_zeroed>
    14fc:	e115                	bnez	a0,1520 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hbf98704c9ff0dbaaE+0x4e>
    14fe:	a809                	j	1510 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hbf98704c9ff0dbaaE+0x3e>
    1500:	ed19                	bnez	a0,151e <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hbf98704c9ff0dbaaE+0x4c>
    1502:	45a1                	li	a1,8
    1504:	8522                	mv	a0,s0
    1506:	fffff097          	auipc	ra,0xfffff
    150a:	5fc080e7          	jalr	1532(ra) # b02 <__rust_alloc>
    150e:	e909                	bnez	a0,1520 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hbf98704c9ff0dbaaE+0x4e>
    1510:	45a1                	li	a1,8
    1512:	8522                	mv	a0,s0
    1514:	00001097          	auipc	ra,0x1
    1518:	932080e7          	jalr	-1742(ra) # 1e46 <_ZN5alloc5alloc18handle_alloc_error17h0809b9ba7eebe66bE>
    151c:	0000                	unimp
    151e:	4521                	li	a0,8
    1520:	00345593          	srli	a1,s0,0x3
    1524:	6402                	ld	s0,0(sp)
    1526:	60a2                	ld	ra,8(sp)
    1528:	0141                	addi	sp,sp,16
    152a:	8082                	ret
    152c:	00001097          	auipc	ra,0x1
    1530:	936080e7          	jalr	-1738(ra) # 1e62 <_ZN5alloc7raw_vec17capacity_overflow17hb114381a505af03eE>
	...

0000000000001536 <_ZN12tornado_user6shared14AddressSpaceId8from_raw17hbb7124f5610e8528E>:
    1536:	8082                	ret

0000000000001538 <_ZN12tornado_user6shared16SharedTaskHandle13should_switch17h3346afc28a4cf301E>:
    1538:	4501                	li	a0,0
    153a:	8082                	ret

000000000000153c <rust_begin_unwind>:
    153c:	9002                	ebreak

000000000000153e <.LBB0_1>:
    153e:	00004517          	auipc	a0,0x4
    1542:	faa50513          	addi	a0,a0,-86 # 54e8 <.Lanon.f915c59a07edaa678460ac0d37a0bb80.0>

0000000000001546 <.LBB0_2>:
    1546:	00004617          	auipc	a2,0x4
    154a:	fe260613          	addi	a2,a2,-30 # 5528 <.Lanon.f915c59a07edaa678460ac0d37a0bb80.2>
    154e:	02800593          	li	a1,40
    1552:	00001097          	auipc	ra,0x1
    1556:	9f0080e7          	jalr	-1552(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
	...

000000000000155c <rust_oom>:
    155c:	9002                	ebreak

000000000000155e <.LBB1_1>:
    155e:	00004517          	auipc	a0,0x4
    1562:	f8a50513          	addi	a0,a0,-118 # 54e8 <.Lanon.f915c59a07edaa678460ac0d37a0bb80.0>

0000000000001566 <.LBB1_2>:
    1566:	00004617          	auipc	a2,0x4
    156a:	fda60613          	addi	a2,a2,-38 # 5540 <.Lanon.f915c59a07edaa678460ac0d37a0bb80.3>
    156e:	02800593          	li	a1,40
    1572:	00001097          	auipc	ra,0x1
    1576:	9d0080e7          	jalr	-1584(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
	...

000000000000157c <_ZN12tornado_user4exit17h19453ae3096d9226E>:
    157c:	0005081b          	sext.w	a6,a0
    1580:	4881                	li	a7,0
    1582:	00000073          	ecall
    1586:	8082                	ret

0000000000001588 <_ZN12tornado_user8do_yield17h86c358fa6dc094c1E>:
    1588:	00121537          	lui	a0,0x121
    158c:	2125089b          	addiw	a7,a0,530
    1590:	4801                	li	a6,0
    1592:	00000073          	ecall
    1596:	8082                	ret

0000000000001598 <__rg_alloc>:
    1598:	0000e617          	auipc	a2,0xe
    159c:	aa060613          	addi	a2,a2,-1376 # f038 <_ZN12tornado_user4HEAP17h1339c640c584f725E>
    15a0:	86ae                	mv	a3,a1
    15a2:	85aa                	mv	a1,a0
    15a4:	8532                	mv	a0,a2
    15a6:	8636                	mv	a2,a3
    15a8:	00001317          	auipc	t1,0x1
    15ac:	81430067          	jr	-2028(t1) # 1dbc <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17hac59b11ac015f0aeE>

00000000000015b0 <__rg_dealloc>:
    15b0:	0000e697          	auipc	a3,0xe
    15b4:	a8868693          	addi	a3,a3,-1400 # f038 <_ZN12tornado_user4HEAP17h1339c640c584f725E>
    15b8:	8732                	mv	a4,a2
    15ba:	862e                	mv	a2,a1
    15bc:	85aa                	mv	a1,a0
    15be:	8536                	mv	a0,a3
    15c0:	86ba                	mv	a3,a4
    15c2:	00001317          	auipc	t1,0x1
    15c6:	83430067          	jr	-1996(t1) # 1df6 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17ha9769221a9c3bda9E>

00000000000015ca <__rg_realloc>:
    15ca:	7179                	addi	sp,sp,-48
    15cc:	f406                	sd	ra,40(sp)
    15ce:	f022                	sd	s0,32(sp)
    15d0:	ec26                	sd	s1,24(sp)
    15d2:	e84a                	sd	s2,16(sp)
    15d4:	e44e                	sd	s3,8(sp)
    15d6:	e052                	sd	s4,0(sp)
    15d8:	84b6                	mv	s1,a3
    15da:	89b2                	mv	s3,a2
    15dc:	8a2e                	mv	s4,a1
    15de:	892a                	mv	s2,a0

00000000000015e0 <.LBB8_5>:
    15e0:	0000e517          	auipc	a0,0xe
    15e4:	a5850513          	addi	a0,a0,-1448 # f038 <_ZN12tornado_user4HEAP17h1339c640c584f725E>
    15e8:	85b6                	mv	a1,a3
    15ea:	00000097          	auipc	ra,0x0
    15ee:	7d2080e7          	jalr	2002(ra) # 1dbc <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17hac59b11ac015f0aeE>
    15f2:	842a                	mv	s0,a0
    15f4:	c515                	beqz	a0,1620 <.LBB8_6+0x16>
    15f6:	0144e363          	bltu	s1,s4,15fc <.LBB8_5+0x1c>
    15fa:	84d2                	mv	s1,s4
    15fc:	8522                	mv	a0,s0
    15fe:	85ca                	mv	a1,s2
    1600:	8626                	mv	a2,s1
    1602:	00003097          	auipc	ra,0x3
    1606:	9f6080e7          	jalr	-1546(ra) # 3ff8 <memcpy>

000000000000160a <.LBB8_6>:
    160a:	0000e517          	auipc	a0,0xe
    160e:	a2e50513          	addi	a0,a0,-1490 # f038 <_ZN12tornado_user4HEAP17h1339c640c584f725E>
    1612:	85ca                	mv	a1,s2
    1614:	8652                	mv	a2,s4
    1616:	86ce                	mv	a3,s3
    1618:	00000097          	auipc	ra,0x0
    161c:	7de080e7          	jalr	2014(ra) # 1df6 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17ha9769221a9c3bda9E>
    1620:	8522                	mv	a0,s0
    1622:	6a02                	ld	s4,0(sp)
    1624:	69a2                	ld	s3,8(sp)
    1626:	6942                	ld	s2,16(sp)
    1628:	64e2                	ld	s1,24(sp)
    162a:	7402                	ld	s0,32(sp)
    162c:	70a2                	ld	ra,40(sp)
    162e:	6145                	addi	sp,sp,48
    1630:	8082                	ret

0000000000001632 <__rg_alloc_zeroed>:
    1632:	1101                	addi	sp,sp,-32
    1634:	ec06                	sd	ra,24(sp)
    1636:	e822                	sd	s0,16(sp)
    1638:	e426                	sd	s1,8(sp)
    163a:	862e                	mv	a2,a1
    163c:	842a                	mv	s0,a0

000000000000163e <.LBB9_3>:
    163e:	0000e517          	auipc	a0,0xe
    1642:	9fa50513          	addi	a0,a0,-1542 # f038 <_ZN12tornado_user4HEAP17h1339c640c584f725E>
    1646:	85a2                	mv	a1,s0
    1648:	00000097          	auipc	ra,0x0
    164c:	774080e7          	jalr	1908(ra) # 1dbc <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17hac59b11ac015f0aeE>
    1650:	84aa                	mv	s1,a0
    1652:	c901                	beqz	a0,1662 <.LBB9_3+0x24>
    1654:	8526                	mv	a0,s1
    1656:	4581                	li	a1,0
    1658:	8622                	mv	a2,s0
    165a:	00003097          	auipc	ra,0x3
    165e:	9b4080e7          	jalr	-1612(ra) # 400e <memset>
    1662:	8526                	mv	a0,s1
    1664:	64a2                	ld	s1,8(sp)
    1666:	6442                	ld	s0,16(sp)
    1668:	60e2                	ld	ra,24(sp)
    166a:	6105                	addi	sp,sp,32
    166c:	8082                	ret

000000000000166e <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE>:
    166e:	715d                	addi	sp,sp,-80
    1670:	e486                	sd	ra,72(sp)
    1672:	e0a2                	sd	s0,64(sp)
    1674:	fc26                	sd	s1,56(sp)
    1676:	f84a                	sd	s2,48(sp)
    1678:	842a                	mv	s0,a0
    167a:	0330000f          	fence	rw,rw
    167e:	6108                	ld	a0,0(a0)
    1680:	0230000f          	fence	r,rw
    1684:	ed41                	bnez	a0,171c <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0xae>
    1686:	4585                	li	a1,1
    1688:	1604352f          	lr.d.aqrl	a0,(s0)
    168c:	e501                	bnez	a0,1694 <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0x26>
    168e:	1eb4362f          	sc.d.aqrl	a2,a1,(s0)
    1692:	fa7d                	bnez	a2,1688 <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0x1a>
    1694:	e541                	bnez	a0,171c <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0xae>
    1696:	e422                	sd	s0,8(sp)
    1698:	00b10823          	sb	a1,16(sp)
    169c:	4521                	li	a0,8
    169e:	4581                	li	a1,0
    16a0:	00000097          	auipc	ra,0x0
    16a4:	e32080e7          	jalr	-462(ra) # 14d2 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17hbf98704c9ff0dbaaE>
    16a8:	84aa                	mv	s1,a0
    16aa:	892e                	mv	s2,a1
    16ac:	01910513          	addi	a0,sp,25
    16b0:	051d                	addi	a0,a0,7
    16b2:	4641                	li	a2,16
    16b4:	4581                	li	a1,0
    16b6:	00003097          	auipc	ra,0x3
    16ba:	958080e7          	jalr	-1704(ra) # 400e <memset>
    16be:	6408                	ld	a0,8(s0)
    16c0:	c115                	beqz	a0,16e4 <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0x76>
    16c2:	01840513          	addi	a0,s0,24
    16c6:	00000097          	auipc	ra,0x0
    16ca:	cca080e7          	jalr	-822(ra) # 1390 <_ZN90_$LT$alloc..collections..vec_deque..VecDeque$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8a627e7196a82204E>
    16ce:	7808                	ld	a0,48(s0)
    16d0:	c911                	beqz	a0,16e4 <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0x76>
    16d2:	00351593          	slli	a1,a0,0x3
    16d6:	c599                	beqz	a1,16e4 <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0x76>
    16d8:	7408                	ld	a0,40(s0)
    16da:	4621                	li	a2,8
    16dc:	fffff097          	auipc	ra,0xfffff
    16e0:	42e080e7          	jalr	1070(ra) # b0a <__rust_dealloc>
    16e4:	4505                	li	a0,1
    16e6:	e408                	sd	a0,8(s0)
    16e8:	00040823          	sb	zero,16(s0)
    16ec:	01140513          	addi	a0,s0,17
    16f0:	01910593          	addi	a1,sp,25
    16f4:	465d                	li	a2,23
    16f6:	00003097          	auipc	ra,0x3
    16fa:	902080e7          	jalr	-1790(ra) # 3ff8 <memcpy>
    16fe:	f404                	sd	s1,40(s0)
    1700:	03243823          	sd	s2,48(s0)
    1704:	00010823          	sb	zero,16(sp)
    1708:	0310000f          	fence	rw,w
    170c:	4509                	li	a0,2
    170e:	e008                	sd	a0,0(s0)
    1710:	0028                	addi	a0,sp,8
    1712:	00000097          	auipc	ra,0x0
    1716:	722080e7          	jalr	1826(ra) # 1e34 <_ZN60_$LT$spin..once..Finish$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb1e30508f786e888E>
    171a:	a005                	j	173a <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0xcc>
    171c:	4585                	li	a1,1
    171e:	00a5cb63          	blt	a1,a0,1734 <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0xc6>
    1722:	02b51463          	bne	a0,a1,174a <.LBB0_14>
    1726:	0330000f          	fence	rw,rw
    172a:	6008                	ld	a0,0(s0)
    172c:	0230000f          	fence	r,rw
    1730:	fea5d9e3          	bge	a1,a0,1722 <_ZN4spin4once13Once$LT$T$GT$9call_once17h18d019345cf754beE+0xb4>
    1734:	4589                	li	a1,2
    1736:	02b51963          	bne	a0,a1,1768 <.LBB0_16>
    173a:	01040513          	addi	a0,s0,16
    173e:	7942                	ld	s2,48(sp)
    1740:	74e2                	ld	s1,56(sp)
    1742:	6406                	ld	s0,64(sp)
    1744:	60a6                	ld	ra,72(sp)
    1746:	6161                	addi	sp,sp,80
    1748:	8082                	ret

000000000000174a <.LBB0_14>:
    174a:	00004517          	auipc	a0,0x4
    174e:	e9650513          	addi	a0,a0,-362 # 55e0 <anon.d80fe1a93814aa1e9fa97fc113a24de8.3.llvm.13337655799424603394>

0000000000001752 <.LBB0_15>:
    1752:	00004617          	auipc	a2,0x4
    1756:	eb660613          	addi	a2,a2,-330 # 5608 <anon.d80fe1a93814aa1e9fa97fc113a24de8.4.llvm.13337655799424603394>
    175a:	02800593          	li	a1,40
    175e:	00000097          	auipc	ra,0x0
    1762:	7e4080e7          	jalr	2020(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
	...

0000000000001768 <.LBB0_16>:
    1768:	00004517          	auipc	a0,0x4
    176c:	df050513          	addi	a0,a0,-528 # 5558 <anon.d80fe1a93814aa1e9fa97fc113a24de8.0.llvm.13337655799424603394>

0000000000001770 <.LBB0_17>:
    1770:	00004617          	auipc	a2,0x4
    1774:	e5860613          	addi	a2,a2,-424 # 55c8 <anon.d80fe1a93814aa1e9fa97fc113a24de8.2.llvm.13337655799424603394>
    1778:	45c5                	li	a1,17
    177a:	00000097          	auipc	ra,0x0
    177e:	7c8080e7          	jalr	1992(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
	...

0000000000001784 <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E>:
    1784:	962e                	add	a2,a2,a1
    1786:	059d                	addi	a1,a1,7
    1788:	99e1                	andi	a1,a1,-8
    178a:	ff867e93          	andi	t4,a2,-8
    178e:	16bee663          	bltu	t4,a1,18fa <.LBB5_15>
    1792:	4701                	li	a4,0
    1794:	00858613          	addi	a2,a1,8
    1798:	12ceef63          	bltu	t4,a2,18d6 <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E+0x152>
    179c:	03f00813          	li	a6,63
    17a0:	4885                	li	a7,1
    17a2:	42fd                	li	t0,31
    17a4:	05555637          	lui	a2,0x5555
    17a8:	5556061b          	addiw	a2,a2,1365
    17ac:	0632                	slli	a2,a2,0xc
    17ae:	55560613          	addi	a2,a2,1365 # 5555555 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x55463e5>
    17b2:	0632                	slli	a2,a2,0xc
    17b4:	55560613          	addi	a2,a2,1365
    17b8:	0632                	slli	a2,a2,0xc
    17ba:	55560313          	addi	t1,a2,1365
    17be:	03333637          	lui	a2,0x3333
    17c2:	3336061b          	addiw	a2,a2,819
    17c6:	0632                	slli	a2,a2,0xc
    17c8:	33360613          	addi	a2,a2,819 # 3333333 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x33241c3>
    17cc:	0632                	slli	a2,a2,0xc
    17ce:	33360613          	addi	a2,a2,819
    17d2:	0632                	slli	a2,a2,0xc
    17d4:	33360f13          	addi	t5,a2,819
    17d8:	00f0f637          	lui	a2,0xf0f
    17dc:	0f16061b          	addiw	a2,a2,241
    17e0:	0632                	slli	a2,a2,0xc
    17e2:	f0f60613          	addi	a2,a2,-241 # f0ef0f <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xeffd9f>
    17e6:	0632                	slli	a2,a2,0xc
    17e8:	0f160613          	addi	a2,a2,241
    17ec:	0632                	slli	a2,a2,0xc
    17ee:	f0f60393          	addi	t2,a2,-241
    17f2:	01010637          	lui	a2,0x1010
    17f6:	1016061b          	addiw	a2,a2,257
    17fa:	0642                	slli	a2,a2,0x10
    17fc:	10160613          	addi	a2,a2,257 # 1010101 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1000f91>
    1800:	0642                	slli	a2,a2,0x10
    1802:	10160e13          	addi	t3,a2,257
    1806:	40b006b3          	neg	a3,a1
    180a:	40be8633          	sub	a2,t4,a1
    180e:	8eed                	and	a3,a3,a1
    1810:	ca59                	beqz	a2,18a6 <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E+0x122>
    1812:	00165793          	srli	a5,a2,0x1
    1816:	8e5d                	or	a2,a2,a5
    1818:	00265793          	srli	a5,a2,0x2
    181c:	8e5d                	or	a2,a2,a5
    181e:	00465793          	srli	a5,a2,0x4
    1822:	8e5d                	or	a2,a2,a5
    1824:	00865793          	srli	a5,a2,0x8
    1828:	8e5d                	or	a2,a2,a5
    182a:	01065793          	srli	a5,a2,0x10
    182e:	8e5d                	or	a2,a2,a5
    1830:	02065793          	srli	a5,a2,0x20
    1834:	8e5d                	or	a2,a2,a5
    1836:	fff64613          	not	a2,a2
    183a:	00165793          	srli	a5,a2,0x1
    183e:	0067f7b3          	and	a5,a5,t1
    1842:	8e1d                	sub	a2,a2,a5
    1844:	01e677b3          	and	a5,a2,t5
    1848:	8209                	srli	a2,a2,0x2
    184a:	01e67633          	and	a2,a2,t5
    184e:	963e                	add	a2,a2,a5
    1850:	00465793          	srli	a5,a2,0x4
    1854:	963e                	add	a2,a2,a5
    1856:	00767633          	and	a2,a2,t2
    185a:	03c60633          	mul	a2,a2,t3
    185e:	9261                	srli	a2,a2,0x38
    1860:	40c80633          	sub	a2,a6,a2
    1864:	00c89633          	sll	a2,a7,a2
    1868:	00d66363          	bltu	a2,a3,186e <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E+0xea>
    186c:	8636                	mv	a2,a3
    186e:	c629                	beqz	a2,18b8 <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E+0x134>
    1870:	fff60693          	addi	a3,a2,-1
    1874:	fff64793          	not	a5,a2
    1878:	8efd                	and	a3,a3,a5
    187a:	0016d793          	srli	a5,a3,0x1
    187e:	0067f7b3          	and	a5,a5,t1
    1882:	8e9d                	sub	a3,a3,a5
    1884:	01e6f7b3          	and	a5,a3,t5
    1888:	8289                	srli	a3,a3,0x2
    188a:	01e6f6b3          	and	a3,a3,t5
    188e:	96be                	add	a3,a3,a5
    1890:	0046d793          	srli	a5,a3,0x4
    1894:	96be                	add	a3,a3,a5
    1896:	0076f6b3          	and	a3,a3,t2
    189a:	03c686b3          	mul	a3,a3,t3
    189e:	92e1                	srli	a3,a3,0x38
    18a0:	02d2f063          	bgeu	t0,a3,18c0 <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E+0x13c>
    18a4:	a83d                	j	18e2 <.LBB5_14>
    18a6:	04000613          	li	a2,64
    18aa:	40c80633          	sub	a2,a6,a2
    18ae:	00c89633          	sll	a2,a7,a2
    18b2:	fad67de3          	bgeu	a2,a3,186c <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E+0xe8>
    18b6:	bf65                	j	186e <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E+0xea>
    18b8:	04000693          	li	a3,64
    18bc:	02d2e363          	bltu	t0,a3,18e2 <.LBB5_14>
    18c0:	068e                	slli	a3,a3,0x3
    18c2:	96aa                	add	a3,a3,a0
    18c4:	629c                	ld	a5,0(a3)
    18c6:	e19c                	sd	a5,0(a1)
    18c8:	e28c                	sd	a1,0(a3)
    18ca:	95b2                	add	a1,a1,a2
    18cc:	00858693          	addi	a3,a1,8
    18d0:	9732                	add	a4,a4,a2
    18d2:	f2defae3          	bgeu	t4,a3,1806 <_ZN22buddy_system_allocator4Heap4init17h7ed312b29ef7fd84E+0x82>
    18d6:	11053583          	ld	a1,272(a0)
    18da:	95ba                	add	a1,a1,a4
    18dc:	10b53823          	sd	a1,272(a0)
    18e0:	8082                	ret

00000000000018e2 <.LBB5_14>:
    18e2:	00004617          	auipc	a2,0x4
    18e6:	de660613          	addi	a2,a2,-538 # 56c8 <.Lanon.9ecd2340286333da0be21283f7862ea4.3>
    18ea:	02000593          	li	a1,32
    18ee:	8536                	mv	a0,a3
    18f0:	00000097          	auipc	ra,0x0
    18f4:	67e080e7          	jalr	1662(ra) # 1f6e <_ZN4core9panicking18panic_bounds_check17hb80951707fb532bdE>
	...

00000000000018fa <.LBB5_15>:
    18fa:	00004517          	auipc	a0,0x4
    18fe:	d2650513          	addi	a0,a0,-730 # 5620 <.Lanon.9ecd2340286333da0be21283f7862ea4.0>

0000000000001902 <.LBB5_16>:
    1902:	00004617          	auipc	a2,0x4
    1906:	dae60613          	addi	a2,a2,-594 # 56b0 <.Lanon.9ecd2340286333da0be21283f7862ea4.2>
    190a:	45f9                	li	a1,30
    190c:	00000097          	auipc	ra,0x0
    1910:	636080e7          	jalr	1590(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
	...

0000000000001916 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE>:
    1916:	4885                	li	a7,1
    1918:	12b8ec63          	bltu	a7,a1,1a50 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0x13a>
    191c:	46a1                	li	a3,8
    191e:	1ec6f863          	bgeu	a3,a2,1b0e <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0x1f8>
    1922:	1f167963          	bgeu	a2,a7,1b14 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0x1fe>
    1926:	1e088a63          	beqz	a7,1b1a <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0x204>
    192a:	fff88613          	addi	a2,a7,-1
    192e:	fff8c693          	not	a3,a7
    1932:	8e75                	and	a2,a2,a3
    1934:	00165693          	srli	a3,a2,0x1
    1938:	05555737          	lui	a4,0x5555
    193c:	5557071b          	addiw	a4,a4,1365
    1940:	0732                	slli	a4,a4,0xc
    1942:	55570713          	addi	a4,a4,1365 # 5555555 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x55463e5>
    1946:	0732                	slli	a4,a4,0xc
    1948:	55570713          	addi	a4,a4,1365
    194c:	0732                	slli	a4,a4,0xc
    194e:	55570713          	addi	a4,a4,1365
    1952:	8ef9                	and	a3,a3,a4
    1954:	8e15                	sub	a2,a2,a3
    1956:	033336b7          	lui	a3,0x3333
    195a:	3336869b          	addiw	a3,a3,819
    195e:	06b2                	slli	a3,a3,0xc
    1960:	33368693          	addi	a3,a3,819 # 3333333 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x33241c3>
    1964:	06b2                	slli	a3,a3,0xc
    1966:	33368693          	addi	a3,a3,819
    196a:	06b2                	slli	a3,a3,0xc
    196c:	33368693          	addi	a3,a3,819
    1970:	00d67733          	and	a4,a2,a3
    1974:	8209                	srli	a2,a2,0x2
    1976:	8e75                	and	a2,a2,a3
    1978:	963a                	add	a2,a2,a4
    197a:	00465693          	srli	a3,a2,0x4
    197e:	9636                	add	a2,a2,a3
    1980:	00f0f6b7          	lui	a3,0xf0f
    1984:	0f16869b          	addiw	a3,a3,241
    1988:	06b2                	slli	a3,a3,0xc
    198a:	f0f68693          	addi	a3,a3,-241 # f0ef0f <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xeffd9f>
    198e:	06b2                	slli	a3,a3,0xc
    1990:	0f168693          	addi	a3,a3,241
    1994:	06b2                	slli	a3,a3,0xc
    1996:	f0f68693          	addi	a3,a3,-241
    199a:	8e75                	and	a2,a2,a3
    199c:	010106b7          	lui	a3,0x1010
    19a0:	1016869b          	addiw	a3,a3,257
    19a4:	06c2                	slli	a3,a3,0x10
    19a6:	10168693          	addi	a3,a3,257 # 1010101 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1000f91>
    19aa:	06c2                	slli	a3,a3,0x10
    19ac:	10168693          	addi	a3,a3,257
    19b0:	02d60633          	mul	a2,a2,a3
    19b4:	03865293          	srli	t0,a2,0x38
    19b8:	02000613          	li	a2,32
    19bc:	8716                	mv	a4,t0
    19be:	00566463          	bltu	a2,t0,19c6 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0xb0>
    19c2:	02000713          	li	a4,32
    19c6:	00329813          	slli	a6,t0,0x3
    19ca:	00a80633          	add	a2,a6,a0
    19ce:	ff060793          	addi	a5,a2,-16
    19d2:	8696                	mv	a3,t0
    19d4:	06d70c63          	beq	a4,a3,1a4c <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0x136>
    19d8:	6b90                	ld	a2,16(a5)
    19da:	0685                	addi	a3,a3,1
    19dc:	07a1                	addi	a5,a5,8
    19de:	da7d                	beqz	a2,19d4 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0xbe>
    19e0:	00128313          	addi	t1,t0,1
    19e4:	02d37c63          	bgeu	t1,a3,1a1c <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0x106>
    19e8:	4e7d                	li	t3,31
    19ea:	4385                	li	t2,1
    19ec:	fff68713          	addi	a4,a3,-1
    19f0:	12ee6d63          	bltu	t3,a4,1b2a <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0x214>
    19f4:	00063e83          	ld	t4,0(a2)
    19f8:	16f9                	addi	a3,a3,-2
    19fa:	01d7b423          	sd	t4,8(a5)
    19fe:	12de6d63          	bltu	t3,a3,1b38 <.LBB6_28>
    1a02:	0007be83          	ld	t4,0(a5)
    1a06:	00d396b3          	sll	a3,t2,a3
    1a0a:	96b2                	add	a3,a3,a2
    1a0c:	01d6b023          	sd	t4,0(a3)
    1a10:	e214                	sd	a3,0(a2)
    1a12:	e390                	sd	a2,0(a5)
    1a14:	17e1                	addi	a5,a5,-8
    1a16:	86ba                	mv	a3,a4
    1a18:	fce36ae3          	bltu	t1,a4,19ec <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0xd6>
    1a1c:	02000613          	li	a2,32
    1a20:	12c2f863          	bgeu	t0,a2,1b50 <.LBB6_29>
    1a24:	010506b3          	add	a3,a0,a6
    1a28:	6290                	ld	a2,0(a3)
    1a2a:	12060f63          	beqz	a2,1b68 <.LBB6_30>
    1a2e:	6218                	ld	a4,0(a2)
    1a30:	e298                	sd	a4,0(a3)
    1a32:	10053683          	ld	a3,256(a0)
    1a36:	10853703          	ld	a4,264(a0)
    1a3a:	95b6                	add	a1,a1,a3
    1a3c:	10b53023          	sd	a1,256(a0)
    1a40:	011705b3          	add	a1,a4,a7
    1a44:	10b53423          	sd	a1,264(a0)
    1a48:	8532                	mv	a0,a2
    1a4a:	8082                	ret
    1a4c:	4501                	li	a0,0
    1a4e:	8082                	ret
    1a50:	fff58693          	addi	a3,a1,-1
    1a54:	0016d713          	srli	a4,a3,0x1
    1a58:	8ed9                	or	a3,a3,a4
    1a5a:	0026d713          	srli	a4,a3,0x2
    1a5e:	8ed9                	or	a3,a3,a4
    1a60:	0046d713          	srli	a4,a3,0x4
    1a64:	8ed9                	or	a3,a3,a4
    1a66:	0086d713          	srli	a4,a3,0x8
    1a6a:	8ed9                	or	a3,a3,a4
    1a6c:	0106d713          	srli	a4,a3,0x10
    1a70:	8ed9                	or	a3,a3,a4
    1a72:	0206d713          	srli	a4,a3,0x20
    1a76:	8ed9                	or	a3,a3,a4
    1a78:	fff6c693          	not	a3,a3
    1a7c:	0016d713          	srli	a4,a3,0x1
    1a80:	055557b7          	lui	a5,0x5555
    1a84:	5557879b          	addiw	a5,a5,1365
    1a88:	07b2                	slli	a5,a5,0xc
    1a8a:	55578793          	addi	a5,a5,1365 # 5555555 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x55463e5>
    1a8e:	07b2                	slli	a5,a5,0xc
    1a90:	55578793          	addi	a5,a5,1365
    1a94:	07b2                	slli	a5,a5,0xc
    1a96:	55578793          	addi	a5,a5,1365
    1a9a:	8f7d                	and	a4,a4,a5
    1a9c:	8e99                	sub	a3,a3,a4
    1a9e:	03333737          	lui	a4,0x3333
    1aa2:	3337071b          	addiw	a4,a4,819
    1aa6:	0732                	slli	a4,a4,0xc
    1aa8:	33370713          	addi	a4,a4,819 # 3333333 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x33241c3>
    1aac:	0732                	slli	a4,a4,0xc
    1aae:	33370713          	addi	a4,a4,819
    1ab2:	0732                	slli	a4,a4,0xc
    1ab4:	33370713          	addi	a4,a4,819
    1ab8:	00e6f7b3          	and	a5,a3,a4
    1abc:	8289                	srli	a3,a3,0x2
    1abe:	8ef9                	and	a3,a3,a4
    1ac0:	96be                	add	a3,a3,a5
    1ac2:	0046d713          	srli	a4,a3,0x4
    1ac6:	96ba                	add	a3,a3,a4
    1ac8:	00f0f737          	lui	a4,0xf0f
    1acc:	0f17071b          	addiw	a4,a4,241
    1ad0:	0732                	slli	a4,a4,0xc
    1ad2:	f0f70713          	addi	a4,a4,-241 # f0ef0f <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xeffd9f>
    1ad6:	0732                	slli	a4,a4,0xc
    1ad8:	0f170713          	addi	a4,a4,241
    1adc:	0732                	slli	a4,a4,0xc
    1ade:	f0f70713          	addi	a4,a4,-241
    1ae2:	8ef9                	and	a3,a3,a4
    1ae4:	01010737          	lui	a4,0x1010
    1ae8:	1017071b          	addiw	a4,a4,257
    1aec:	0742                	slli	a4,a4,0x10
    1aee:	10170713          	addi	a4,a4,257 # 1010101 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1000f91>
    1af2:	0742                	slli	a4,a4,0x10
    1af4:	10170713          	addi	a4,a4,257
    1af8:	02e686b3          	mul	a3,a3,a4
    1afc:	92e1                	srli	a3,a3,0x38
    1afe:	577d                	li	a4,-1
    1b00:	00d756b3          	srl	a3,a4,a3
    1b04:	00168893          	addi	a7,a3,1
    1b08:	46a1                	li	a3,8
    1b0a:	e0c6ece3          	bltu	a3,a2,1922 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0xc>
    1b0e:	4621                	li	a2,8
    1b10:	e1166be3          	bltu	a2,a7,1926 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0x10>
    1b14:	88b2                	mv	a7,a2
    1b16:	e0089ae3          	bnez	a7,192a <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0x14>
    1b1a:	04000293          	li	t0,64
    1b1e:	02000613          	li	a2,32
    1b22:	8716                	mv	a4,t0
    1b24:	e8567fe3          	bgeu	a2,t0,19c2 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0xac>
    1b28:	bd79                	j	19c6 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE+0xb0>
    1b2a:	fff68513          	addi	a0,a3,-1

0000000000001b2e <.LBB6_27>:
    1b2e:	00004617          	auipc	a2,0x4
    1b32:	bb260613          	addi	a2,a2,-1102 # 56e0 <.Lanon.9ecd2340286333da0be21283f7862ea4.5>
    1b36:	a031                	j	1b42 <.LBB6_28+0xa>

0000000000001b38 <.LBB6_28>:
    1b38:	00004617          	auipc	a2,0x4
    1b3c:	bc060613          	addi	a2,a2,-1088 # 56f8 <.Lanon.9ecd2340286333da0be21283f7862ea4.6>
    1b40:	557d                	li	a0,-1
    1b42:	02000593          	li	a1,32
    1b46:	00000097          	auipc	ra,0x0
    1b4a:	428080e7          	jalr	1064(ra) # 1f6e <_ZN4core9panicking18panic_bounds_check17hb80951707fb532bdE>
	...

0000000000001b50 <.LBB6_29>:
    1b50:	00004617          	auipc	a2,0x4
    1b54:	bc060613          	addi	a2,a2,-1088 # 5710 <.Lanon.9ecd2340286333da0be21283f7862ea4.7>
    1b58:	02000593          	li	a1,32
    1b5c:	8516                	mv	a0,t0
    1b5e:	00000097          	auipc	ra,0x0
    1b62:	410080e7          	jalr	1040(ra) # 1f6e <_ZN4core9panicking18panic_bounds_check17hb80951707fb532bdE>
	...

0000000000001b68 <.LBB6_30>:
    1b68:	00004517          	auipc	a0,0x4
    1b6c:	bc050513          	addi	a0,a0,-1088 # 5728 <.Lanon.9ecd2340286333da0be21283f7862ea4.8>

0000000000001b70 <.LBB6_31>:
    1b70:	00004617          	auipc	a2,0x4
    1b74:	be060613          	addi	a2,a2,-1056 # 5750 <.Lanon.9ecd2340286333da0be21283f7862ea4.9>
    1b78:	02800593          	li	a1,40
    1b7c:	00000097          	auipc	ra,0x0
    1b80:	38c080e7          	jalr	908(ra) # 1f08 <_ZN4core6option13expect_failed17h61f5cec36dd7f056E>
	...

0000000000001b86 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE>:
    1b86:	4285                	li	t0,1
    1b88:	12c2e563          	bltu	t0,a2,1cb2 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0x12c>
    1b8c:	4721                	li	a4,8
    1b8e:	1ed77263          	bgeu	a4,a3,1d72 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0x1ec>
    1b92:	1e56f363          	bgeu	a3,t0,1d78 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0x1f2>
    1b96:	1e028463          	beqz	t0,1d7e <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0x1f8>
    1b9a:	fff28693          	addi	a3,t0,-1
    1b9e:	fff2c713          	not	a4,t0
    1ba2:	8ef9                	and	a3,a3,a4
    1ba4:	0016d713          	srli	a4,a3,0x1
    1ba8:	055557b7          	lui	a5,0x5555
    1bac:	5557879b          	addiw	a5,a5,1365
    1bb0:	07b2                	slli	a5,a5,0xc
    1bb2:	55578793          	addi	a5,a5,1365 # 5555555 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x55463e5>
    1bb6:	07b2                	slli	a5,a5,0xc
    1bb8:	55578793          	addi	a5,a5,1365
    1bbc:	07b2                	slli	a5,a5,0xc
    1bbe:	55578793          	addi	a5,a5,1365
    1bc2:	8f7d                	and	a4,a4,a5
    1bc4:	8e99                	sub	a3,a3,a4
    1bc6:	03333737          	lui	a4,0x3333
    1bca:	3337071b          	addiw	a4,a4,819
    1bce:	0732                	slli	a4,a4,0xc
    1bd0:	33370713          	addi	a4,a4,819 # 3333333 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x33241c3>
    1bd4:	0732                	slli	a4,a4,0xc
    1bd6:	33370713          	addi	a4,a4,819
    1bda:	0732                	slli	a4,a4,0xc
    1bdc:	33370713          	addi	a4,a4,819
    1be0:	00e6f7b3          	and	a5,a3,a4
    1be4:	8289                	srli	a3,a3,0x2
    1be6:	8ef9                	and	a3,a3,a4
    1be8:	96be                	add	a3,a3,a5
    1bea:	0046d713          	srli	a4,a3,0x4
    1bee:	96ba                	add	a3,a3,a4
    1bf0:	00f0f737          	lui	a4,0xf0f
    1bf4:	0f17071b          	addiw	a4,a4,241
    1bf8:	0732                	slli	a4,a4,0xc
    1bfa:	f0f70713          	addi	a4,a4,-241 # f0ef0f <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xeffd9f>
    1bfe:	0732                	slli	a4,a4,0xc
    1c00:	0f170713          	addi	a4,a4,241
    1c04:	0732                	slli	a4,a4,0xc
    1c06:	f0f70713          	addi	a4,a4,-241
    1c0a:	8ef9                	and	a3,a3,a4
    1c0c:	01010737          	lui	a4,0x1010
    1c10:	1017071b          	addiw	a4,a4,257
    1c14:	0742                	slli	a4,a4,0x10
    1c16:	10170713          	addi	a4,a4,257 # 1010101 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1000f91>
    1c1a:	0742                	slli	a4,a4,0x10
    1c1c:	10170713          	addi	a4,a4,257
    1c20:	02e686b3          	mul	a3,a3,a4
    1c24:	0386d313          	srli	t1,a3,0x38
    1c28:	487d                	li	a6,31
    1c2a:	14686f63          	bltu	a6,t1,1d88 <.LBB7_23>
    1c2e:	00331693          	slli	a3,t1,0x3
    1c32:	00d503b3          	add	t2,a0,a3
    1c36:	0003b683          	ld	a3,0(t2)
    1c3a:	e194                	sd	a3,0(a1)
    1c3c:	00b3b023          	sd	a1,0(t2)
    1c40:	4885                	li	a7,1
    1c42:	006896b3          	sll	a3,a7,t1
    1c46:	00b6c733          	xor	a4,a3,a1
    1c4a:	8e2e                	mv	t3,a1
    1c4c:	869e                	mv	a3,t2
    1c4e:	87ae                	mv	a5,a1
    1c50:	00b70863          	beq	a4,a1,1c60 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0xda>
    1c54:	86be                	mv	a3,a5
    1c56:	639c                	ld	a5,0(a5)
    1c58:	c3a9                	beqz	a5,1c9a <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0x114>
    1c5a:	fef71de3          	bne	a4,a5,1c54 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0xce>
    1c5e:	8e3a                	mv	t3,a4
    1c60:	000e3783          	ld	a5,0(t3)
    1c64:	e29c                	sd	a5,0(a3)
    1c66:	0003b683          	ld	a3,0(t2)
    1c6a:	c681                	beqz	a3,1c72 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0xec>
    1c6c:	6294                	ld	a3,0(a3)
    1c6e:	00d3b023          	sd	a3,0(t2)
    1c72:	00b76363          	bltu	a4,a1,1c78 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0xf2>
    1c76:	872e                	mv	a4,a1
    1c78:	13030463          	beq	t1,a6,1da0 <.LBB7_24>
    1c7c:	85ba                	mv	a1,a4
    1c7e:	0305                	addi	t1,t1,1
    1c80:	00331693          	slli	a3,t1,0x3
    1c84:	00d503b3          	add	t2,a0,a3
    1c88:	0003b683          	ld	a3,0(t2)
    1c8c:	e314                	sd	a3,0(a4)
    1c8e:	00e3b023          	sd	a4,0(t2)
    1c92:	006896b3          	sll	a3,a7,t1
    1c96:	8f35                	xor	a4,a4,a3
    1c98:	f9cd                	bnez	a1,1c4a <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0xc4>
    1c9a:	10053583          	ld	a1,256(a0)
    1c9e:	10853683          	ld	a3,264(a0)
    1ca2:	8d91                	sub	a1,a1,a2
    1ca4:	10b53023          	sd	a1,256(a0)
    1ca8:	405685b3          	sub	a1,a3,t0
    1cac:	10b53423          	sd	a1,264(a0)
    1cb0:	8082                	ret
    1cb2:	fff60713          	addi	a4,a2,-1
    1cb6:	00175793          	srli	a5,a4,0x1
    1cba:	8f5d                	or	a4,a4,a5
    1cbc:	00275793          	srli	a5,a4,0x2
    1cc0:	8f5d                	or	a4,a4,a5
    1cc2:	00475793          	srli	a5,a4,0x4
    1cc6:	8f5d                	or	a4,a4,a5
    1cc8:	00875793          	srli	a5,a4,0x8
    1ccc:	8f5d                	or	a4,a4,a5
    1cce:	01075793          	srli	a5,a4,0x10
    1cd2:	8f5d                	or	a4,a4,a5
    1cd4:	02075793          	srli	a5,a4,0x20
    1cd8:	8f5d                	or	a4,a4,a5
    1cda:	fff74713          	not	a4,a4
    1cde:	00175813          	srli	a6,a4,0x1
    1ce2:	055557b7          	lui	a5,0x5555
    1ce6:	5557879b          	addiw	a5,a5,1365
    1cea:	07b2                	slli	a5,a5,0xc
    1cec:	55578793          	addi	a5,a5,1365 # 5555555 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x55463e5>
    1cf0:	07b2                	slli	a5,a5,0xc
    1cf2:	55578793          	addi	a5,a5,1365
    1cf6:	07b2                	slli	a5,a5,0xc
    1cf8:	55578793          	addi	a5,a5,1365
    1cfc:	00f877b3          	and	a5,a6,a5
    1d00:	8f1d                	sub	a4,a4,a5
    1d02:	033337b7          	lui	a5,0x3333
    1d06:	3337879b          	addiw	a5,a5,819
    1d0a:	07b2                	slli	a5,a5,0xc
    1d0c:	33378793          	addi	a5,a5,819 # 3333333 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x33241c3>
    1d10:	07b2                	slli	a5,a5,0xc
    1d12:	33378793          	addi	a5,a5,819
    1d16:	07b2                	slli	a5,a5,0xc
    1d18:	33378793          	addi	a5,a5,819
    1d1c:	00f77833          	and	a6,a4,a5
    1d20:	8309                	srli	a4,a4,0x2
    1d22:	8f7d                	and	a4,a4,a5
    1d24:	9742                	add	a4,a4,a6
    1d26:	00475793          	srli	a5,a4,0x4
    1d2a:	973e                	add	a4,a4,a5
    1d2c:	00f0f7b7          	lui	a5,0xf0f
    1d30:	0f17879b          	addiw	a5,a5,241
    1d34:	07b2                	slli	a5,a5,0xc
    1d36:	f0f78793          	addi	a5,a5,-241 # f0ef0f <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xeffd9f>
    1d3a:	07b2                	slli	a5,a5,0xc
    1d3c:	0f178793          	addi	a5,a5,241
    1d40:	07b2                	slli	a5,a5,0xc
    1d42:	f0f78793          	addi	a5,a5,-241
    1d46:	8f7d                	and	a4,a4,a5
    1d48:	010107b7          	lui	a5,0x1010
    1d4c:	1017879b          	addiw	a5,a5,257
    1d50:	07c2                	slli	a5,a5,0x10
    1d52:	10178793          	addi	a5,a5,257 # 1010101 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1000f91>
    1d56:	07c2                	slli	a5,a5,0x10
    1d58:	10178793          	addi	a5,a5,257
    1d5c:	02f70733          	mul	a4,a4,a5
    1d60:	9361                	srli	a4,a4,0x38
    1d62:	57fd                	li	a5,-1
    1d64:	00e7d733          	srl	a4,a5,a4
    1d68:	00170293          	addi	t0,a4,1
    1d6c:	4721                	li	a4,8
    1d6e:	e2d762e3          	bltu	a4,a3,1b92 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0xc>
    1d72:	46a1                	li	a3,8
    1d74:	e256e1e3          	bltu	a3,t0,1b96 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0x10>
    1d78:	82b6                	mv	t0,a3
    1d7a:	e20290e3          	bnez	t0,1b9a <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0x14>
    1d7e:	04000313          	li	t1,64
    1d82:	487d                	li	a6,31
    1d84:	ea6875e3          	bgeu	a6,t1,1c2e <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE+0xa8>

0000000000001d88 <.LBB7_23>:
    1d88:	00004617          	auipc	a2,0x4
    1d8c:	9e060613          	addi	a2,a2,-1568 # 5768 <.Lanon.9ecd2340286333da0be21283f7862ea4.10>
    1d90:	02000593          	li	a1,32
    1d94:	851a                	mv	a0,t1
    1d96:	00000097          	auipc	ra,0x0
    1d9a:	1d8080e7          	jalr	472(ra) # 1f6e <_ZN4core9panicking18panic_bounds_check17hb80951707fb532bdE>
	...

0000000000001da0 <.LBB7_24>:
    1da0:	00004617          	auipc	a2,0x4
    1da4:	9e060613          	addi	a2,a2,-1568 # 5780 <.Lanon.9ecd2340286333da0be21283f7862ea4.11>
    1da8:	02000513          	li	a0,32
    1dac:	02000593          	li	a1,32
    1db0:	00000097          	auipc	ra,0x0
    1db4:	1be080e7          	jalr	446(ra) # 1f6e <_ZN4core9panicking18panic_bounds_check17hb80951707fb532bdE>
	...

0000000000001dba <_ZN78_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..ops..deref..Deref$GT$5deref17h9a8f4477244c971aE>:
    1dba:	8082                	ret

0000000000001dbc <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17hac59b11ac015f0aeE>:
    1dbc:	1101                	addi	sp,sp,-32
    1dbe:	ec06                	sd	ra,24(sp)
    1dc0:	e822                	sd	s0,16(sp)
    1dc2:	e426                	sd	s1,8(sp)
    1dc4:	842a                	mv	s0,a0
    1dc6:	4505                	li	a0,1
    1dc8:	00a434af          	amoadd.d	s1,a0,(s0)
    1dcc:	6408                	ld	a0,8(s0)
    1dce:	0230000f          	fence	r,rw
    1dd2:	fe951de3          	bne	a0,s1,1dcc <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17hac59b11ac015f0aeE+0x10>
    1dd6:	01040513          	addi	a0,s0,16
    1dda:	00000097          	auipc	ra,0x0
    1dde:	b3c080e7          	jalr	-1220(ra) # 1916 <_ZN22buddy_system_allocator4Heap5alloc17h4119708cedbb18caE>
    1de2:	00148593          	addi	a1,s1,1
    1de6:	0310000f          	fence	rw,w
    1dea:	e40c                	sd	a1,8(s0)
    1dec:	64a2                	ld	s1,8(sp)
    1dee:	6442                	ld	s0,16(sp)
    1df0:	60e2                	ld	ra,24(sp)
    1df2:	6105                	addi	sp,sp,32
    1df4:	8082                	ret

0000000000001df6 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17ha9769221a9c3bda9E>:
    1df6:	1101                	addi	sp,sp,-32
    1df8:	ec06                	sd	ra,24(sp)
    1dfa:	e822                	sd	s0,16(sp)
    1dfc:	e426                	sd	s1,8(sp)
    1dfe:	842a                	mv	s0,a0
    1e00:	4505                	li	a0,1
    1e02:	00a434af          	amoadd.d	s1,a0,(s0)
    1e06:	6408                	ld	a0,8(s0)
    1e08:	0230000f          	fence	r,rw
    1e0c:	fe951de3          	bne	a0,s1,1e06 <_ZN87_$LT$buddy_system_allocator..LockedHeap$u20$as$u20$core..alloc..global..GlobalAlloc$GT$7dealloc17ha9769221a9c3bda9E+0x10>
    1e10:	01040513          	addi	a0,s0,16
    1e14:	00000097          	auipc	ra,0x0
    1e18:	d72080e7          	jalr	-654(ra) # 1b86 <_ZN22buddy_system_allocator4Heap7dealloc17h7906327fb1d1e79aE>
    1e1c:	00148513          	addi	a0,s1,1
    1e20:	0310000f          	fence	rw,w
    1e24:	e408                	sd	a0,8(s0)
    1e26:	64a2                	ld	s1,8(sp)
    1e28:	6442                	ld	s0,16(sp)
    1e2a:	60e2                	ld	ra,24(sp)
    1e2c:	6105                	addi	sp,sp,32
    1e2e:	8082                	ret

0000000000001e30 <_ZN4woke8WakerRef11new_unowned17h502a062306ae1e43E>:
    1e30:	8082                	ret

0000000000001e32 <_ZN58_$LT$woke..WakerRef$u20$as$u20$core..ops..deref..Deref$GT$5deref17h2fd191855ca923cdE>:
    1e32:	8082                	ret

0000000000001e34 <_ZN60_$LT$spin..once..Finish$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb1e30508f786e888E>:
    1e34:	00854583          	lbu	a1,8(a0)
    1e38:	c591                	beqz	a1,1e44 <_ZN60_$LT$spin..once..Finish$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb1e30508f786e888E+0x10>
    1e3a:	6108                	ld	a0,0(a0)
    1e3c:	0310000f          	fence	rw,w
    1e40:	458d                	li	a1,3
    1e42:	e10c                	sd	a1,0(a0)
    1e44:	8082                	ret

0000000000001e46 <_ZN5alloc5alloc18handle_alloc_error17h0809b9ba7eebe66bE>:
    1e46:	1141                	addi	sp,sp,-16
    1e48:	e406                	sd	ra,8(sp)
    1e4a:	fffff097          	auipc	ra,0xfffff
    1e4e:	cd8080e7          	jalr	-808(ra) # b22 <__rust_alloc_error_handler>
	...

0000000000001e54 <__rg_oom>:
    1e54:	1141                	addi	sp,sp,-16
    1e56:	e406                	sd	ra,8(sp)
    1e58:	fffff097          	auipc	ra,0xfffff
    1e5c:	704080e7          	jalr	1796(ra) # 155c <rust_oom>
	...

0000000000001e62 <_ZN5alloc7raw_vec17capacity_overflow17hb114381a505af03eE>:
    1e62:	1141                	addi	sp,sp,-16
    1e64:	e406                	sd	ra,8(sp)

0000000000001e66 <.LBB23_1>:
    1e66:	00004517          	auipc	a0,0x4
    1e6a:	94e50513          	addi	a0,a0,-1714 # 57b4 <.Lanon.ef313fdb91fd0bc7c97c02b65db35f6c.19>

0000000000001e6e <.LBB23_2>:
    1e6e:	00004617          	auipc	a2,0x4
    1e72:	95a60613          	addi	a2,a2,-1702 # 57c8 <.Lanon.ef313fdb91fd0bc7c97c02b65db35f6c.20>
    1e76:	45c5                	li	a1,17
    1e78:	00000097          	auipc	ra,0x0
    1e7c:	0ca080e7          	jalr	202(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
	...

0000000000001e82 <_ZN4core3ops8function6FnOnce9call_once17hb87d53624de904adE>:
    1e82:	6108                	ld	a0,0(a0)
    1e84:	a001                	j	1e84 <_ZN4core3ops8function6FnOnce9call_once17hb87d53624de904adE+0x2>

0000000000001e86 <_ZN4core3ptr102drop_in_place$LT$$RF$core..iter..adapters..copied..Copied$LT$core..slice..iter..Iter$LT$u8$GT$$GT$$GT$17h6e888a30b6524f70E>:
    1e86:	8082                	ret

0000000000001e88 <_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hf9fc33902e262dfeE>:
    1e88:	715d                	addi	sp,sp,-80
    1e8a:	e486                	sd	ra,72(sp)
    1e8c:	e0a2                	sd	s0,64(sp)
    1e8e:	fc26                	sd	s1,56(sp)
    1e90:	842e                	mv	s0,a1
    1e92:	84aa                	mv	s1,a0
    1e94:	00002097          	auipc	ra,0x2
    1e98:	b7e080e7          	jalr	-1154(ra) # 3a12 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E>
    1e9c:	e51d                	bnez	a0,1eca <.LBB76_5+0x18>
    1e9e:	7008                	ld	a0,32(s0)
    1ea0:	740c                	ld	a1,40(s0)

0000000000001ea2 <.LBB76_4>:
    1ea2:	00004617          	auipc	a2,0x4
    1ea6:	94660613          	addi	a2,a2,-1722 # 57e8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.135>
    1eaa:	e432                	sd	a2,8(sp)
    1eac:	4605                	li	a2,1
    1eae:	e832                	sd	a2,16(sp)
    1eb0:	ec02                	sd	zero,24(sp)

0000000000001eb2 <.LBB76_5>:
    1eb2:	00004617          	auipc	a2,0x4
    1eb6:	92e60613          	addi	a2,a2,-1746 # 57e0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.134>
    1eba:	f432                	sd	a2,40(sp)
    1ebc:	f802                	sd	zero,48(sp)
    1ebe:	0030                	addi	a2,sp,8
    1ec0:	00001097          	auipc	ra,0x1
    1ec4:	822080e7          	jalr	-2014(ra) # 26e2 <_ZN4core3fmt5write17h6dfe8a21eb788173E>
    1ec8:	c519                	beqz	a0,1ed6 <.LBB76_5+0x24>
    1eca:	4505                	li	a0,1
    1ecc:	74e2                	ld	s1,56(sp)
    1ece:	6406                	ld	s0,64(sp)
    1ed0:	60a6                	ld	ra,72(sp)
    1ed2:	6161                	addi	sp,sp,80
    1ed4:	8082                	ret
    1ed6:	00848513          	addi	a0,s1,8
    1eda:	85a2                	mv	a1,s0
    1edc:	74e2                	ld	s1,56(sp)
    1ede:	6406                	ld	s0,64(sp)
    1ee0:	60a6                	ld	ra,72(sp)
    1ee2:	6161                	addi	sp,sp,80
    1ee4:	00002317          	auipc	t1,0x2
    1ee8:	b2e30067          	jr	-1234(t1) # 3a12 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E>

0000000000001eec <_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hbe6462e270c1539cE>:
    1eec:	fccdf537          	lui	a0,0xfccdf
    1ef0:	8835051b          	addiw	a0,a0,-1917
    1ef4:	0532                	slli	a0,a0,0xc
    1ef6:	fd950513          	addi	a0,a0,-39 # fffffffffccdefd9 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xfffffffffcccfe69>
    1efa:	0536                	slli	a0,a0,0xd
    1efc:	e0f50513          	addi	a0,a0,-497
    1f00:	0532                	slli	a0,a0,0xc
    1f02:	e1450513          	addi	a0,a0,-492
    1f06:	8082                	ret

0000000000001f08 <_ZN4core6option13expect_failed17h61f5cec36dd7f056E>:
    1f08:	711d                	addi	sp,sp,-96
    1f0a:	ec86                	sd	ra,88(sp)
    1f0c:	e42a                	sd	a0,8(sp)
    1f0e:	e82e                	sd	a1,16(sp)
    1f10:	0028                	addi	a0,sp,8
    1f12:	e4aa                	sd	a0,72(sp)

0000000000001f14 <.LBB117_1>:
    1f14:	00002517          	auipc	a0,0x2
    1f18:	f6850513          	addi	a0,a0,-152 # 3e7c <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hc6a9db97781cac35E>
    1f1c:	e8aa                	sd	a0,80(sp)

0000000000001f1e <.LBB117_2>:
    1f1e:	00004517          	auipc	a0,0x4
    1f22:	90a50513          	addi	a0,a0,-1782 # 5828 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.170>
    1f26:	ec2a                	sd	a0,24(sp)
    1f28:	4505                	li	a0,1
    1f2a:	f02a                	sd	a0,32(sp)
    1f2c:	f402                	sd	zero,40(sp)
    1f2e:	00ac                	addi	a1,sp,72
    1f30:	fc2e                	sd	a1,56(sp)
    1f32:	e0aa                	sd	a0,64(sp)
    1f34:	0828                	addi	a0,sp,24
    1f36:	85b2                	mv	a1,a2
    1f38:	00000097          	auipc	ra,0x0
    1f3c:	076080e7          	jalr	118(ra) # 1fae <_ZN4core9panicking9panic_fmt17ha96d96e8fa0883ecE>
	...

0000000000001f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>:
    1f42:	715d                	addi	sp,sp,-80
    1f44:	e486                	sd	ra,72(sp)
    1f46:	fc2a                	sd	a0,56(sp)
    1f48:	e0ae                	sd	a1,64(sp)
    1f4a:	1828                	addi	a0,sp,56
    1f4c:	e42a                	sd	a0,8(sp)
    1f4e:	4505                	li	a0,1
    1f50:	e82a                	sd	a0,16(sp)
    1f52:	ec02                	sd	zero,24(sp)

0000000000001f54 <.LBB129_1>:
    1f54:	00004517          	auipc	a0,0x4
    1f58:	88c50513          	addi	a0,a0,-1908 # 57e0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.134>
    1f5c:	f42a                	sd	a0,40(sp)
    1f5e:	f802                	sd	zero,48(sp)
    1f60:	0028                	addi	a0,sp,8
    1f62:	85b2                	mv	a1,a2
    1f64:	00000097          	auipc	ra,0x0
    1f68:	04a080e7          	jalr	74(ra) # 1fae <_ZN4core9panicking9panic_fmt17ha96d96e8fa0883ecE>
	...

0000000000001f6e <_ZN4core9panicking18panic_bounds_check17hb80951707fb532bdE>:
    1f6e:	7159                	addi	sp,sp,-112
    1f70:	f486                	sd	ra,104(sp)
    1f72:	e42a                	sd	a0,8(sp)
    1f74:	e82e                	sd	a1,16(sp)
    1f76:	0808                	addi	a0,sp,16
    1f78:	e4aa                	sd	a0,72(sp)

0000000000001f7a <.LBB130_1>:
    1f7a:	00002517          	auipc	a0,0x2
    1f7e:	d3e50513          	addi	a0,a0,-706 # 3cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>
    1f82:	e8aa                	sd	a0,80(sp)
    1f84:	002c                	addi	a1,sp,8
    1f86:	ecae                	sd	a1,88(sp)
    1f88:	f0aa                	sd	a0,96(sp)

0000000000001f8a <.LBB130_2>:
    1f8a:	00004517          	auipc	a0,0x4
    1f8e:	94e50513          	addi	a0,a0,-1714 # 58d8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.182>
    1f92:	ec2a                	sd	a0,24(sp)
    1f94:	4509                	li	a0,2
    1f96:	f02a                	sd	a0,32(sp)
    1f98:	f402                	sd	zero,40(sp)
    1f9a:	00ac                	addi	a1,sp,72
    1f9c:	fc2e                	sd	a1,56(sp)
    1f9e:	e0aa                	sd	a0,64(sp)
    1fa0:	0828                	addi	a0,sp,24
    1fa2:	85b2                	mv	a1,a2
    1fa4:	00000097          	auipc	ra,0x0
    1fa8:	00a080e7          	jalr	10(ra) # 1fae <_ZN4core9panicking9panic_fmt17ha96d96e8fa0883ecE>
	...

0000000000001fae <_ZN4core9panicking9panic_fmt17ha96d96e8fa0883ecE>:
    1fae:	7179                	addi	sp,sp,-48
    1fb0:	f406                	sd	ra,40(sp)

0000000000001fb2 <.LBB131_1>:
    1fb2:	00004617          	auipc	a2,0x4
    1fb6:	82e60613          	addi	a2,a2,-2002 # 57e0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.134>
    1fba:	e432                	sd	a2,8(sp)

0000000000001fbc <.LBB131_2>:
    1fbc:	00004617          	auipc	a2,0x4
    1fc0:	88460613          	addi	a2,a2,-1916 # 5840 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.173>
    1fc4:	e832                	sd	a2,16(sp)
    1fc6:	ec2a                	sd	a0,24(sp)
    1fc8:	f02e                	sd	a1,32(sp)
    1fca:	0028                	addi	a0,sp,8
    1fcc:	fffff097          	auipc	ra,0xfffff
    1fd0:	570080e7          	jalr	1392(ra) # 153c <rust_begin_unwind>
	...

0000000000001fd6 <_ZN4core9panicking13assert_failed5inner17h080205f7502c6bf2E>:
    1fd6:	7115                	addi	sp,sp,-224
    1fd8:	ed86                	sd	ra,216(sp)
    1fda:	e42e                	sd	a1,8(sp)
    1fdc:	e832                	sd	a2,16(sp)
    1fde:	ec36                	sd	a3,24(sp)
    1fe0:	f03a                	sd	a4,32(sp)
    1fe2:	e511                	bnez	a0,1fee <.LBB133_8>

0000000000001fe4 <.LBB133_7>:
    1fe4:	00004517          	auipc	a0,0x4
    1fe8:	91650513          	addi	a0,a0,-1770 # 58fa <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.185>
    1fec:	a029                	j	1ff6 <.LBB133_8+0x8>

0000000000001fee <.LBB133_8>:
    1fee:	00004517          	auipc	a0,0x4
    1ff2:	90a50513          	addi	a0,a0,-1782 # 58f8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.184>
    1ff6:	638c                	ld	a1,0(a5)
    1ff8:	f42a                	sd	a0,40(sp)
    1ffa:	4509                	li	a0,2
    1ffc:	f82a                	sd	a0,48(sp)
    1ffe:	ed95                	bnez	a1,203a <.LBB133_11+0x18>
    2000:	1028                	addi	a0,sp,40
    2002:	f4aa                	sd	a0,104(sp)

0000000000002004 <.LBB133_9>:
    2004:	00002517          	auipc	a0,0x2
    2008:	e7850513          	addi	a0,a0,-392 # 3e7c <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hc6a9db97781cac35E>
    200c:	f8aa                	sd	a0,112(sp)
    200e:	0028                	addi	a0,sp,8
    2010:	fcaa                	sd	a0,120(sp)

0000000000002012 <.LBB133_10>:
    2012:	00002517          	auipc	a0,0x2
    2016:	e6250513          	addi	a0,a0,-414 # 3e74 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h93049563736a027cE>
    201a:	e12a                	sd	a0,128(sp)
    201c:	082c                	addi	a1,sp,24
    201e:	e52e                	sd	a1,136(sp)
    2020:	e92a                	sd	a0,144(sp)

0000000000002022 <.LBB133_11>:
    2022:	00004517          	auipc	a0,0x4
    2026:	96650513          	addi	a0,a0,-1690 # 5988 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.191>
    202a:	f52a                	sd	a0,168(sp)
    202c:	4511                	li	a0,4
    202e:	f92a                	sd	a0,176(sp)
    2030:	fd02                	sd	zero,184(sp)
    2032:	10a8                	addi	a0,sp,104
    2034:	e5aa                	sd	a0,200(sp)
    2036:	450d                	li	a0,3
    2038:	a085                	j	2098 <.LBB133_15+0x16>
    203a:	7788                	ld	a0,40(a5)
    203c:	738c                	ld	a1,32(a5)
    203e:	f0aa                	sd	a0,96(sp)
    2040:	ecae                	sd	a1,88(sp)
    2042:	6f88                	ld	a0,24(a5)
    2044:	6b8c                	ld	a1,16(a5)
    2046:	6790                	ld	a2,8(a5)
    2048:	6394                	ld	a3,0(a5)
    204a:	e8aa                	sd	a0,80(sp)
    204c:	e4ae                	sd	a1,72(sp)
    204e:	e0b2                	sd	a2,64(sp)
    2050:	fc36                	sd	a3,56(sp)
    2052:	1028                	addi	a0,sp,40
    2054:	f4aa                	sd	a0,104(sp)

0000000000002056 <.LBB133_12>:
    2056:	00002517          	auipc	a0,0x2
    205a:	e2650513          	addi	a0,a0,-474 # 3e7c <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hc6a9db97781cac35E>
    205e:	f8aa                	sd	a0,112(sp)
    2060:	0028                	addi	a0,sp,8
    2062:	fcaa                	sd	a0,120(sp)

0000000000002064 <.LBB133_13>:
    2064:	00002517          	auipc	a0,0x2
    2068:	e1050513          	addi	a0,a0,-496 # 3e74 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h93049563736a027cE>
    206c:	e12a                	sd	a0,128(sp)
    206e:	082c                	addi	a1,sp,24
    2070:	e52e                	sd	a1,136(sp)
    2072:	e92a                	sd	a0,144(sp)
    2074:	1828                	addi	a0,sp,56
    2076:	ed2a                	sd	a0,152(sp)

0000000000002078 <.LBB133_14>:
    2078:	00000517          	auipc	a0,0x0
    207c:	63850513          	addi	a0,a0,1592 # 26b0 <_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h2b9d0b3964d282a9E>
    2080:	f12a                	sd	a0,160(sp)

0000000000002082 <.LBB133_15>:
    2082:	00004517          	auipc	a0,0x4
    2086:	8b650513          	addi	a0,a0,-1866 # 5938 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.190>
    208a:	f52a                	sd	a0,168(sp)
    208c:	4515                	li	a0,5
    208e:	f92a                	sd	a0,176(sp)
    2090:	fd02                	sd	zero,184(sp)
    2092:	10a8                	addi	a0,sp,104
    2094:	e5aa                	sd	a0,200(sp)
    2096:	4511                	li	a0,4
    2098:	e9aa                	sd	a0,208(sp)
    209a:	1128                	addi	a0,sp,168
    209c:	85c2                	mv	a1,a6
    209e:	00000097          	auipc	ra,0x0
    20a2:	f10080e7          	jalr	-240(ra) # 1fae <_ZN4core9panicking9panic_fmt17ha96d96e8fa0883ecE>
	...

00000000000020a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>:
    20a8:	715d                	addi	sp,sp,-80
    20aa:	e486                	sd	ra,72(sp)
    20ac:	e0a2                	sd	s0,64(sp)
    20ae:	fc26                	sd	s1,56(sp)
    20b0:	f84a                	sd	s2,48(sp)
    20b2:	f44e                	sd	s3,40(sp)
    20b4:	f052                	sd	s4,32(sp)
    20b6:	ec56                	sd	s5,24(sp)
    20b8:	e85a                	sd	s6,16(sp)
    20ba:	e45e                	sd	s7,8(sp)
    20bc:	e062                	sd	s8,0(sp)
    20be:	ca7d                	beqz	a2,21b4 <.LBB135_30+0x1a>
    20c0:	84b2                	mv	s1,a2
    20c2:	89ae                	mv	s3,a1
    20c4:	892a                	mv	s2,a0
    20c6:	4a3d                	li	s4,15
    20c8:	4aa9                	li	s5,10
    20ca:	fbf00b13          	li	s6,-65
    20ce:	4b85                	li	s7,1
    20d0:	a809                	j	20e2 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E+0x3a>
    20d2:	6d94                	ld	a3,24(a1)
    20d4:	85ce                	mv	a1,s3
    20d6:	8622                	mv	a2,s0
    20d8:	9682                	jalr	a3
    20da:	ed79                	bnez	a0,21b8 <.LBB135_30+0x1e>
    20dc:	8c81                	sub	s1,s1,s0
    20de:	89e2                	mv	s3,s8
    20e0:	c8f1                	beqz	s1,21b4 <.LBB135_30+0x1a>
    20e2:	01093503          	ld	a0,16(s2)
    20e6:	00054503          	lbu	a0,0(a0)
    20ea:	cd09                	beqz	a0,2104 <.LBB135_29+0xe>
    20ec:	00893583          	ld	a1,8(s2)
    20f0:	00093503          	ld	a0,0(s2)
    20f4:	6d94                	ld	a3,24(a1)

00000000000020f6 <.LBB135_29>:
    20f6:	00003597          	auipc	a1,0x3
    20fa:	0f258593          	addi	a1,a1,242 # 51e8 <anon.be5ea868b286bb36e8d2b81fd8475abb.0.llvm.4157917051624895175+0x20>
    20fe:	4611                	li	a2,4
    2100:	9682                	jalr	a3
    2102:	e95d                	bnez	a0,21b8 <.LBB135_30+0x1e>
    2104:	4401                	li	s0,0
    2106:	8626                	mv	a2,s1
    2108:	a029                	j	2112 <.LBB135_29+0x1c>
    210a:	40848633          	sub	a2,s1,s0
    210e:	0484e963          	bltu	s1,s0,2160 <.LBB135_29+0x6a>
    2112:	008985b3          	add	a1,s3,s0
    2116:	02ca6063          	bltu	s4,a2,2136 <.LBB135_29+0x40>
    211a:	c239                	beqz	a2,2160 <.LBB135_29+0x6a>
    211c:	4601                	li	a2,0
    211e:	40848533          	sub	a0,s1,s0
    2122:	00c586b3          	add	a3,a1,a2
    2126:	0006c683          	lbu	a3,0(a3)
    212a:	01568e63          	beq	a3,s5,2146 <.LBB135_29+0x50>
    212e:	0605                	addi	a2,a2,1
    2130:	fec519e3          	bne	a0,a2,2122 <.LBB135_29+0x2c>
    2134:	a035                	j	2160 <.LBB135_29+0x6a>
    2136:	4529                	li	a0,10
    2138:	00001097          	auipc	ra,0x1
    213c:	f84080e7          	jalr	-124(ra) # 30bc <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E>
    2140:	03751063          	bne	a0,s7,2160 <.LBB135_29+0x6a>
    2144:	862e                	mv	a2,a1
    2146:	00860533          	add	a0,a2,s0
    214a:	00150413          	addi	s0,a0,1
    214e:	fa957ee3          	bgeu	a0,s1,210a <.LBB135_29+0x14>
    2152:	954e                	add	a0,a0,s3
    2154:	00054503          	lbu	a0,0(a0)
    2158:	fb5519e3          	bne	a0,s5,210a <.LBB135_29+0x14>
    215c:	4505                	li	a0,1
    215e:	a019                	j	2164 <.LBB135_29+0x6e>
    2160:	4501                	li	a0,0
    2162:	8426                	mv	s0,s1
    2164:	01093583          	ld	a1,16(s2)
    2168:	00a58023          	sb	a0,0(a1)
    216c:	00093503          	ld	a0,0(s2)
    2170:	00893583          	ld	a1,8(s2)
    2174:	00898c33          	add	s8,s3,s0
    2178:	f4848de3          	beq	s1,s0,20d2 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E+0x2a>
    217c:	04947b63          	bgeu	s0,s1,21d2 <.LBB135_31>
    2180:	000c0603          	lb	a2,0(s8)
    2184:	04cb5763          	bge	s6,a2,21d2 <.LBB135_31>
    2188:	6d94                	ld	a3,24(a1)
    218a:	85ce                	mv	a1,s3
    218c:	8622                	mv	a2,s0
    218e:	9682                	jalr	a3
    2190:	e505                	bnez	a0,21b8 <.LBB135_30+0x1e>
    2192:	000c0503          	lb	a0,0(s8)
    2196:	f4ab43e3          	blt	s6,a0,20dc <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E+0x34>

000000000000219a <.LBB135_30>:
    219a:	00004717          	auipc	a4,0x4
    219e:	87670713          	addi	a4,a4,-1930 # 5a10 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.196>
    21a2:	854e                	mv	a0,s3
    21a4:	85a6                	mv	a1,s1
    21a6:	8622                	mv	a2,s0
    21a8:	86a6                	mv	a3,s1
    21aa:	00001097          	auipc	ra,0x1
    21ae:	0da080e7          	jalr	218(ra) # 3284 <_ZN4core3str16slice_error_fail17h4f668c8ce31695ebE>
    21b2:	0000                	unimp
    21b4:	4501                	li	a0,0
    21b6:	a011                	j	21ba <.LBB135_30+0x20>
    21b8:	4505                	li	a0,1
    21ba:	6c02                	ld	s8,0(sp)
    21bc:	6ba2                	ld	s7,8(sp)
    21be:	6b42                	ld	s6,16(sp)
    21c0:	6ae2                	ld	s5,24(sp)
    21c2:	7a02                	ld	s4,32(sp)
    21c4:	79a2                	ld	s3,40(sp)
    21c6:	7942                	ld	s2,48(sp)
    21c8:	74e2                	ld	s1,56(sp)
    21ca:	6406                	ld	s0,64(sp)
    21cc:	60a6                	ld	ra,72(sp)
    21ce:	6161                	addi	sp,sp,80
    21d0:	8082                	ret

00000000000021d2 <.LBB135_31>:
    21d2:	00004717          	auipc	a4,0x4
    21d6:	82670713          	addi	a4,a4,-2010 # 59f8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.195>
    21da:	854e                	mv	a0,s3
    21dc:	85a6                	mv	a1,s1
    21de:	4601                	li	a2,0
    21e0:	86a2                	mv	a3,s0
    21e2:	00001097          	auipc	ra,0x1
    21e6:	0a2080e7          	jalr	162(ra) # 3284 <_ZN4core3str16slice_error_fail17h4f668c8ce31695ebE>
	...

00000000000021ec <_ZN4core3fmt8builders10DebugTuple5field17he8ff51e3f51afcb6E>:
    21ec:	7175                	addi	sp,sp,-144
    21ee:	e506                	sd	ra,136(sp)
    21f0:	e122                	sd	s0,128(sp)
    21f2:	fca6                	sd	s1,120(sp)
    21f4:	f8ca                	sd	s2,112(sp)
    21f6:	f4ce                	sd	s3,104(sp)
    21f8:	842a                	mv	s0,a0
    21fa:	01054503          	lbu	a0,16(a0)
    21fe:	4485                	li	s1,1
    2200:	e179                	bnez	a0,22c6 <.LBB139_18+0x24>
    2202:	89b2                	mv	s3,a2
    2204:	892e                	mv	s2,a1
    2206:	6008                	ld	a0,0(s0)
    2208:	03056583          	lwu	a1,48(a0)
    220c:	6410                	ld	a2,8(s0)
    220e:	0045f693          	andi	a3,a1,4
    2212:	00163613          	seqz	a2,a2
    2216:	e699                	bnez	a3,2224 <.LBB139_14+0xa>
    2218:	e649                	bnez	a2,22a2 <.LBB139_18>

000000000000221a <.LBB139_14>:
    221a:	00004597          	auipc	a1,0x4
    221e:	81058593          	addi	a1,a1,-2032 # 5a2a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.199>
    2222:	a061                	j	22aa <.LBB139_18+0x8>
    2224:	ce11                	beqz	a2,2240 <.LBB139_15+0x14>
    2226:	750c                	ld	a1,40(a0)
    2228:	7108                	ld	a0,32(a0)
    222a:	6d94                	ld	a3,24(a1)

000000000000222c <.LBB139_15>:
    222c:	00004597          	auipc	a1,0x4
    2230:	80058593          	addi	a1,a1,-2048 # 5a2c <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.206>
    2234:	4609                	li	a2,2
    2236:	9682                	jalr	a3
    2238:	e559                	bnez	a0,22c6 <.LBB139_18+0x24>
    223a:	6008                	ld	a0,0(s0)
    223c:	03056583          	lwu	a1,48(a0)
    2240:	4485                	li	s1,1
    2242:	029103a3          	sb	s1,39(sp)
    2246:	7110                	ld	a2,32(a0)
    2248:	7514                	ld	a3,40(a0)
    224a:	e432                	sd	a2,8(sp)
    224c:	e836                	sd	a3,16(sp)
    224e:	02710613          	addi	a2,sp,39
    2252:	ec32                	sd	a2,24(sp)
    2254:	5950                	lw	a2,52(a0)
    2256:	03850683          	lb	a3,56(a0)
    225a:	6118                	ld	a4,0(a0)
    225c:	651c                	ld	a5,8(a0)
    225e:	01053803          	ld	a6,16(a0)
    2262:	6d08                	ld	a0,24(a0)
    2264:	ccae                	sw	a1,88(sp)
    2266:	ceb2                	sw	a2,92(sp)
    2268:	06d10023          	sb	a3,96(sp)
    226c:	f43a                	sd	a4,40(sp)
    226e:	f83e                	sd	a5,48(sp)
    2270:	fc42                	sd	a6,56(sp)
    2272:	e0aa                	sd	a0,64(sp)
    2274:	0028                	addi	a0,sp,8
    2276:	0189b603          	ld	a2,24(s3)
    227a:	e4aa                	sd	a0,72(sp)

000000000000227c <.LBB139_16>:
    227c:	00003517          	auipc	a0,0x3
    2280:	74c50513          	addi	a0,a0,1868 # 59c8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.193>
    2284:	e8aa                	sd	a0,80(sp)
    2286:	102c                	addi	a1,sp,40
    2288:	854a                	mv	a0,s2
    228a:	9602                	jalr	a2
    228c:	ed0d                	bnez	a0,22c6 <.LBB139_18+0x24>
    228e:	65c6                	ld	a1,80(sp)
    2290:	6526                	ld	a0,72(sp)
    2292:	6d94                	ld	a3,24(a1)

0000000000002294 <.LBB139_17>:
    2294:	00003597          	auipc	a1,0x3
    2298:	79458593          	addi	a1,a1,1940 # 5a28 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.198>
    229c:	4609                	li	a2,2
    229e:	9682                	jalr	a3
    22a0:	a015                	j	22c4 <.LBB139_18+0x22>

00000000000022a2 <.LBB139_18>:
    22a2:	00003597          	auipc	a1,0x3
    22a6:	78c58593          	addi	a1,a1,1932 # 5a2e <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.207>
    22aa:	7514                	ld	a3,40(a0)
    22ac:	7108                	ld	a0,32(a0)
    22ae:	6e94                	ld	a3,24(a3)
    22b0:	4709                	li	a4,2
    22b2:	40c70633          	sub	a2,a4,a2
    22b6:	9682                	jalr	a3
    22b8:	e519                	bnez	a0,22c6 <.LBB139_18+0x24>
    22ba:	600c                	ld	a1,0(s0)
    22bc:	0189b603          	ld	a2,24(s3)
    22c0:	854a                	mv	a0,s2
    22c2:	9602                	jalr	a2
    22c4:	84aa                	mv	s1,a0
    22c6:	6408                	ld	a0,8(s0)
    22c8:	00940823          	sb	s1,16(s0)
    22cc:	0505                	addi	a0,a0,1
    22ce:	e408                	sd	a0,8(s0)
    22d0:	8522                	mv	a0,s0
    22d2:	79a6                	ld	s3,104(sp)
    22d4:	7946                	ld	s2,112(sp)
    22d6:	74e6                	ld	s1,120(sp)
    22d8:	640a                	ld	s0,128(sp)
    22da:	60aa                	ld	ra,136(sp)
    22dc:	6149                	addi	sp,sp,144
    22de:	8082                	ret

00000000000022e0 <_ZN4core3fmt8builders10DebugTuple6finish17h9892d8aebff080cbE>:
    22e0:	1101                	addi	sp,sp,-32
    22e2:	ec06                	sd	ra,24(sp)
    22e4:	e822                	sd	s0,16(sp)
    22e6:	e426                	sd	s1,8(sp)
    22e8:	842a                	mv	s0,a0
    22ea:	650c                	ld	a1,8(a0)
    22ec:	01054503          	lbu	a0,16(a0)
    22f0:	c5b9                	beqz	a1,233e <.LBB140_10+0x14>
    22f2:	4485                	li	s1,1
    22f4:	e131                	bnez	a0,2338 <.LBB140_10+0xe>
    22f6:	4505                	li	a0,1
    22f8:	02a59563          	bne	a1,a0,2322 <.LBB140_9+0x10>
    22fc:	01144503          	lbu	a0,17(s0)
    2300:	c10d                	beqz	a0,2322 <.LBB140_9+0x10>
    2302:	6008                	ld	a0,0(s0)
    2304:	03054583          	lbu	a1,48(a0)
    2308:	8991                	andi	a1,a1,4
    230a:	ed81                	bnez	a1,2322 <.LBB140_9+0x10>
    230c:	750c                	ld	a1,40(a0)
    230e:	7108                	ld	a0,32(a0)
    2310:	6d94                	ld	a3,24(a1)

0000000000002312 <.LBB140_9>:
    2312:	00003597          	auipc	a1,0x3
    2316:	71d58593          	addi	a1,a1,1821 # 5a2f <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.208>
    231a:	4605                	li	a2,1
    231c:	4485                	li	s1,1
    231e:	9682                	jalr	a3
    2320:	ed01                	bnez	a0,2338 <.LBB140_10+0xe>
    2322:	6008                	ld	a0,0(s0)
    2324:	750c                	ld	a1,40(a0)
    2326:	7108                	ld	a0,32(a0)
    2328:	6d94                	ld	a3,24(a1)

000000000000232a <.LBB140_10>:
    232a:	00003597          	auipc	a1,0x3
    232e:	70658593          	addi	a1,a1,1798 # 5a30 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.209>
    2332:	4605                	li	a2,1
    2334:	9682                	jalr	a3
    2336:	84aa                	mv	s1,a0
    2338:	00940823          	sb	s1,16(s0)
    233c:	8526                	mv	a0,s1
    233e:	00a03533          	snez	a0,a0
    2342:	64a2                	ld	s1,8(sp)
    2344:	6442                	ld	s0,16(sp)
    2346:	60e2                	ld	ra,24(sp)
    2348:	6105                	addi	sp,sp,32
    234a:	8082                	ret

000000000000234c <_ZN4core3fmt8builders10DebugInner5entry17h5f1f1e852caf27ccE>:
    234c:	7175                	addi	sp,sp,-144
    234e:	e506                	sd	ra,136(sp)
    2350:	e122                	sd	s0,128(sp)
    2352:	fca6                	sd	s1,120(sp)
    2354:	f8ca                	sd	s2,112(sp)
    2356:	f4ce                	sd	s3,104(sp)
    2358:	f0d2                	sd	s4,96(sp)
    235a:	842a                	mv	s0,a0
    235c:	00854503          	lbu	a0,8(a0)
    2360:	4a05                	li	s4,1
    2362:	4485                	li	s1,1
    2364:	ed5d                	bnez	a0,2422 <.LBB141_16+0xe>
    2366:	89b2                	mv	s3,a2
    2368:	892e                	mv	s2,a1
    236a:	600c                	ld	a1,0(s0)
    236c:	0305e503          	lwu	a0,48(a1)
    2370:	00944603          	lbu	a2,9(s0)
    2374:	00457693          	andi	a3,a0,4
    2378:	00163613          	seqz	a2,a2
    237c:	e29d                	bnez	a3,23a2 <.LBB141_13+0x1c>
    237e:	ee09                	bnez	a2,2398 <.LBB141_13+0x12>
    2380:	7590                	ld	a2,40(a1)
    2382:	7188                	ld	a0,32(a1)
    2384:	6e14                	ld	a3,24(a2)

0000000000002386 <.LBB141_13>:
    2386:	00003597          	auipc	a1,0x3
    238a:	6a458593          	addi	a1,a1,1700 # 5a2a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.199>
    238e:	4609                	li	a2,2
    2390:	9682                	jalr	a3
    2392:	4485                	li	s1,1
    2394:	e559                	bnez	a0,2422 <.LBB141_16+0xe>
    2396:	600c                	ld	a1,0(s0)
    2398:	0189b603          	ld	a2,24(s3)
    239c:	854a                	mv	a0,s2
    239e:	9602                	jalr	a2
    23a0:	a041                	j	2420 <.LBB141_16+0xc>
    23a2:	ce19                	beqz	a2,23c0 <.LBB141_14+0x16>
    23a4:	7590                	ld	a2,40(a1)
    23a6:	7188                	ld	a0,32(a1)
    23a8:	6e14                	ld	a3,24(a2)

00000000000023aa <.LBB141_14>:
    23aa:	00003597          	auipc	a1,0x3
    23ae:	68758593          	addi	a1,a1,1671 # 5a31 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.210>
    23b2:	4605                	li	a2,1
    23b4:	4485                	li	s1,1
    23b6:	9682                	jalr	a3
    23b8:	e52d                	bnez	a0,2422 <.LBB141_16+0xe>
    23ba:	600c                	ld	a1,0(s0)
    23bc:	0305e503          	lwu	a0,48(a1)
    23c0:	4485                	li	s1,1
    23c2:	00910fa3          	sb	s1,31(sp)
    23c6:	7190                	ld	a2,32(a1)
    23c8:	7594                	ld	a3,40(a1)
    23ca:	e032                	sd	a2,0(sp)
    23cc:	e436                	sd	a3,8(sp)
    23ce:	01f10613          	addi	a2,sp,31
    23d2:	e832                	sd	a2,16(sp)
    23d4:	59d0                	lw	a2,52(a1)
    23d6:	03858683          	lb	a3,56(a1)
    23da:	6198                	ld	a4,0(a1)
    23dc:	659c                	ld	a5,8(a1)
    23de:	0105b803          	ld	a6,16(a1)
    23e2:	6d8c                	ld	a1,24(a1)
    23e4:	c8aa                	sw	a0,80(sp)
    23e6:	cab2                	sw	a2,84(sp)
    23e8:	04d10c23          	sb	a3,88(sp)
    23ec:	f03a                	sd	a4,32(sp)
    23ee:	f43e                	sd	a5,40(sp)
    23f0:	f842                	sd	a6,48(sp)
    23f2:	fc2e                	sd	a1,56(sp)
    23f4:	850a                	mv	a0,sp
    23f6:	0189b603          	ld	a2,24(s3)
    23fa:	e0aa                	sd	a0,64(sp)

00000000000023fc <.LBB141_15>:
    23fc:	00003517          	auipc	a0,0x3
    2400:	5cc50513          	addi	a0,a0,1484 # 59c8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.193>
    2404:	e4aa                	sd	a0,72(sp)
    2406:	100c                	addi	a1,sp,32
    2408:	854a                	mv	a0,s2
    240a:	9602                	jalr	a2
    240c:	e919                	bnez	a0,2422 <.LBB141_16+0xe>
    240e:	65a6                	ld	a1,72(sp)
    2410:	6506                	ld	a0,64(sp)
    2412:	6d94                	ld	a3,24(a1)

0000000000002414 <.LBB141_16>:
    2414:	00003597          	auipc	a1,0x3
    2418:	61458593          	addi	a1,a1,1556 # 5a28 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.198>
    241c:	4609                	li	a2,2
    241e:	9682                	jalr	a3
    2420:	84aa                	mv	s1,a0
    2422:	00940423          	sb	s1,8(s0)
    2426:	014404a3          	sb	s4,9(s0)
    242a:	7a06                	ld	s4,96(sp)
    242c:	79a6                	ld	s3,104(sp)
    242e:	7946                	ld	s2,112(sp)
    2430:	74e6                	ld	s1,120(sp)
    2432:	640a                	ld	s0,128(sp)
    2434:	60aa                	ld	ra,136(sp)
    2436:	6149                	addi	sp,sp,144
    2438:	8082                	ret

000000000000243a <_ZN4core3fmt8builders8DebugSet5entry17hfe13052758799ed9E>:
    243a:	1141                	addi	sp,sp,-16
    243c:	e406                	sd	ra,8(sp)
    243e:	e022                	sd	s0,0(sp)
    2440:	842a                	mv	s0,a0
    2442:	00000097          	auipc	ra,0x0
    2446:	f0a080e7          	jalr	-246(ra) # 234c <_ZN4core3fmt8builders10DebugInner5entry17h5f1f1e852caf27ccE>
    244a:	8522                	mv	a0,s0
    244c:	6402                	ld	s0,0(sp)
    244e:	60a2                	ld	ra,8(sp)
    2450:	0141                	addi	sp,sp,16
    2452:	8082                	ret

0000000000002454 <_ZN4core3fmt8builders9DebugList6finish17h92fc23c61d612d60E>:
    2454:	00854583          	lbu	a1,8(a0)
    2458:	c199                	beqz	a1,245e <_ZN4core3fmt8builders9DebugList6finish17h92fc23c61d612d60E+0xa>
    245a:	4505                	li	a0,1
    245c:	8082                	ret
    245e:	6108                	ld	a0,0(a0)
    2460:	750c                	ld	a1,40(a0)
    2462:	7108                	ld	a0,32(a0)
    2464:	6d9c                	ld	a5,24(a1)

0000000000002466 <.LBB144_3>:
    2466:	00003597          	auipc	a1,0x3
    246a:	5cd58593          	addi	a1,a1,1485 # 5a33 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.216>
    246e:	4605                	li	a2,1
    2470:	8782                	jr	a5

0000000000002472 <_ZN4core3fmt5Write10write_char17h5f3737ec4340f247E>:
    2472:	1141                	addi	sp,sp,-16
    2474:	e406                	sd	ra,8(sp)
    2476:	862e                	mv	a2,a1
    2478:	2581                	sext.w	a1,a1
    247a:	08000693          	li	a3,128
    247e:	c202                	sw	zero,4(sp)
    2480:	00d5fd63          	bgeu	a1,a3,249a <_ZN4core3fmt5Write10write_char17h5f3737ec4340f247E+0x28>
    2484:	004c                	addi	a1,sp,4
    2486:	00c10223          	sb	a2,4(sp)
    248a:	4605                	li	a2,1
    248c:	00000097          	auipc	ra,0x0
    2490:	c1c080e7          	jalr	-996(ra) # 20a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>
    2494:	60a2                	ld	ra,8(sp)
    2496:	0141                	addi	sp,sp,16
    2498:	8082                	ret
    249a:	00b6559b          	srliw	a1,a2,0xb
    249e:	e595                	bnez	a1,24ca <_ZN4core3fmt5Write10write_char17h5f3737ec4340f247E+0x58>
    24a0:	004c                	addi	a1,sp,4
    24a2:	00665693          	srli	a3,a2,0x6
    24a6:	0c06e693          	ori	a3,a3,192
    24aa:	00d10223          	sb	a3,4(sp)
    24ae:	03f67613          	andi	a2,a2,63
    24b2:	08066613          	ori	a2,a2,128
    24b6:	00c102a3          	sb	a2,5(sp)
    24ba:	4609                	li	a2,2
    24bc:	00000097          	auipc	ra,0x0
    24c0:	bec080e7          	jalr	-1044(ra) # 20a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>
    24c4:	60a2                	ld	ra,8(sp)
    24c6:	0141                	addi	sp,sp,16
    24c8:	8082                	ret
    24ca:	0106569b          	srliw	a3,a2,0x10
    24ce:	004c                	addi	a1,sp,4
    24d0:	ee8d                	bnez	a3,250a <_ZN4core3fmt5Write10write_char17h5f3737ec4340f247E+0x98>
    24d2:	00c6569b          	srliw	a3,a2,0xc
    24d6:	0e06e693          	ori	a3,a3,224
    24da:	00d10223          	sb	a3,4(sp)
    24de:	0066569b          	srliw	a3,a2,0x6
    24e2:	03f6f693          	andi	a3,a3,63
    24e6:	0806e693          	ori	a3,a3,128
    24ea:	00d102a3          	sb	a3,5(sp)
    24ee:	03f67613          	andi	a2,a2,63
    24f2:	08066613          	ori	a2,a2,128
    24f6:	00c10323          	sb	a2,6(sp)
    24fa:	460d                	li	a2,3
    24fc:	00000097          	auipc	ra,0x0
    2500:	bac080e7          	jalr	-1108(ra) # 20a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>
    2504:	60a2                	ld	ra,8(sp)
    2506:	0141                	addi	sp,sp,16
    2508:	8082                	ret
    250a:	0126569b          	srliw	a3,a2,0x12
    250e:	0f06e693          	ori	a3,a3,240
    2512:	00d10223          	sb	a3,4(sp)
    2516:	00c6569b          	srliw	a3,a2,0xc
    251a:	03f6f693          	andi	a3,a3,63
    251e:	0806e693          	ori	a3,a3,128
    2522:	00d102a3          	sb	a3,5(sp)
    2526:	0066569b          	srliw	a3,a2,0x6
    252a:	03f6f693          	andi	a3,a3,63
    252e:	0806e693          	ori	a3,a3,128
    2532:	00d10323          	sb	a3,6(sp)
    2536:	03f67613          	andi	a2,a2,63
    253a:	08066613          	ori	a2,a2,128
    253e:	00c103a3          	sb	a2,7(sp)
    2542:	4611                	li	a2,4
    2544:	00000097          	auipc	ra,0x0
    2548:	b64080e7          	jalr	-1180(ra) # 20a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>
    254c:	60a2                	ld	ra,8(sp)
    254e:	0141                	addi	sp,sp,16
    2550:	8082                	ret

0000000000002552 <_ZN4core3fmt5Write9write_fmt17hef72637a41ccea53E>:
    2552:	7139                	addi	sp,sp,-64
    2554:	fc06                	sd	ra,56(sp)
    2556:	7590                	ld	a2,40(a1)
    2558:	7194                	ld	a3,32(a1)
    255a:	e02a                	sd	a0,0(sp)
    255c:	f832                	sd	a2,48(sp)
    255e:	f436                	sd	a3,40(sp)
    2560:	6d88                	ld	a0,24(a1)
    2562:	6990                	ld	a2,16(a1)
    2564:	6594                	ld	a3,8(a1)
    2566:	618c                	ld	a1,0(a1)
    2568:	f02a                	sd	a0,32(sp)
    256a:	ec32                	sd	a2,24(sp)
    256c:	e836                	sd	a3,16(sp)
    256e:	e42e                	sd	a1,8(sp)

0000000000002570 <.LBB162_1>:
    2570:	00003597          	auipc	a1,0x3
    2574:	5c858593          	addi	a1,a1,1480 # 5b38 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.231>
    2578:	850a                	mv	a0,sp
    257a:	0030                	addi	a2,sp,8
    257c:	00000097          	auipc	ra,0x0
    2580:	166080e7          	jalr	358(ra) # 26e2 <_ZN4core3fmt5write17h6dfe8a21eb788173E>
    2584:	70e2                	ld	ra,56(sp)
    2586:	6121                	addi	sp,sp,64
    2588:	8082                	ret

000000000000258a <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hca84e052dfb1177cE>:
    258a:	6108                	ld	a0,0(a0)
    258c:	00000317          	auipc	t1,0x0
    2590:	b1c30067          	jr	-1252(t1) # 20a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>

0000000000002594 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hcb5ce952e07ae217E>:
    2594:	1141                	addi	sp,sp,-16
    2596:	e406                	sd	ra,8(sp)
    2598:	862e                	mv	a2,a1
    259a:	6108                	ld	a0,0(a0)
    259c:	2581                	sext.w	a1,a1
    259e:	08000693          	li	a3,128
    25a2:	c202                	sw	zero,4(sp)
    25a4:	00d5fd63          	bgeu	a1,a3,25be <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hcb5ce952e07ae217E+0x2a>
    25a8:	004c                	addi	a1,sp,4
    25aa:	00c10223          	sb	a2,4(sp)
    25ae:	4605                	li	a2,1
    25b0:	00000097          	auipc	ra,0x0
    25b4:	af8080e7          	jalr	-1288(ra) # 20a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>
    25b8:	60a2                	ld	ra,8(sp)
    25ba:	0141                	addi	sp,sp,16
    25bc:	8082                	ret
    25be:	00b6559b          	srliw	a1,a2,0xb
    25c2:	e595                	bnez	a1,25ee <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hcb5ce952e07ae217E+0x5a>
    25c4:	004c                	addi	a1,sp,4
    25c6:	00665693          	srli	a3,a2,0x6
    25ca:	0c06e693          	ori	a3,a3,192
    25ce:	00d10223          	sb	a3,4(sp)
    25d2:	03f67613          	andi	a2,a2,63
    25d6:	08066613          	ori	a2,a2,128
    25da:	00c102a3          	sb	a2,5(sp)
    25de:	4609                	li	a2,2
    25e0:	00000097          	auipc	ra,0x0
    25e4:	ac8080e7          	jalr	-1336(ra) # 20a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>
    25e8:	60a2                	ld	ra,8(sp)
    25ea:	0141                	addi	sp,sp,16
    25ec:	8082                	ret
    25ee:	0106569b          	srliw	a3,a2,0x10
    25f2:	004c                	addi	a1,sp,4
    25f4:	ee8d                	bnez	a3,262e <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hcb5ce952e07ae217E+0x9a>
    25f6:	00c6569b          	srliw	a3,a2,0xc
    25fa:	0e06e693          	ori	a3,a3,224
    25fe:	00d10223          	sb	a3,4(sp)
    2602:	0066569b          	srliw	a3,a2,0x6
    2606:	03f6f693          	andi	a3,a3,63
    260a:	0806e693          	ori	a3,a3,128
    260e:	00d102a3          	sb	a3,5(sp)
    2612:	03f67613          	andi	a2,a2,63
    2616:	08066613          	ori	a2,a2,128
    261a:	00c10323          	sb	a2,6(sp)
    261e:	460d                	li	a2,3
    2620:	00000097          	auipc	ra,0x0
    2624:	a88080e7          	jalr	-1400(ra) # 20a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>
    2628:	60a2                	ld	ra,8(sp)
    262a:	0141                	addi	sp,sp,16
    262c:	8082                	ret
    262e:	0126569b          	srliw	a3,a2,0x12
    2632:	0f06e693          	ori	a3,a3,240
    2636:	00d10223          	sb	a3,4(sp)
    263a:	00c6569b          	srliw	a3,a2,0xc
    263e:	03f6f693          	andi	a3,a3,63
    2642:	0806e693          	ori	a3,a3,128
    2646:	00d102a3          	sb	a3,5(sp)
    264a:	0066569b          	srliw	a3,a2,0x6
    264e:	03f6f693          	andi	a3,a3,63
    2652:	0806e693          	ori	a3,a3,128
    2656:	00d10323          	sb	a3,6(sp)
    265a:	03f67613          	andi	a2,a2,63
    265e:	08066613          	ori	a2,a2,128
    2662:	00c103a3          	sb	a2,7(sp)
    2666:	4611                	li	a2,4
    2668:	00000097          	auipc	ra,0x0
    266c:	a40080e7          	jalr	-1472(ra) # 20a8 <_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h63c6080cd49c3cc6E>
    2670:	60a2                	ld	ra,8(sp)
    2672:	0141                	addi	sp,sp,16
    2674:	8082                	ret

0000000000002676 <_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h0355fdc3959420e0E>:
    2676:	7139                	addi	sp,sp,-64
    2678:	fc06                	sd	ra,56(sp)
    267a:	6108                	ld	a0,0(a0)
    267c:	7590                	ld	a2,40(a1)
    267e:	7194                	ld	a3,32(a1)
    2680:	e02a                	sd	a0,0(sp)
    2682:	f832                	sd	a2,48(sp)
    2684:	f436                	sd	a3,40(sp)
    2686:	6d88                	ld	a0,24(a1)
    2688:	6990                	ld	a2,16(a1)
    268a:	6594                	ld	a3,8(a1)
    268c:	618c                	ld	a1,0(a1)
    268e:	f02a                	sd	a0,32(sp)
    2690:	ec32                	sd	a2,24(sp)
    2692:	e836                	sd	a3,16(sp)
    2694:	e42e                	sd	a1,8(sp)

0000000000002696 <.LBB165_1>:
    2696:	00003597          	auipc	a1,0x3
    269a:	4a258593          	addi	a1,a1,1186 # 5b38 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.231>
    269e:	850a                	mv	a0,sp
    26a0:	0030                	addi	a2,sp,8
    26a2:	00000097          	auipc	ra,0x0
    26a6:	040080e7          	jalr	64(ra) # 26e2 <_ZN4core3fmt5write17h6dfe8a21eb788173E>
    26aa:	70e2                	ld	ra,56(sp)
    26ac:	6121                	addi	sp,sp,64
    26ae:	8082                	ret

00000000000026b0 <_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h2b9d0b3964d282a9E>:
    26b0:	7139                	addi	sp,sp,-64
    26b2:	fc06                	sd	ra,56(sp)
    26b4:	7510                	ld	a2,40(a0)
    26b6:	7118                	ld	a4,32(a0)
    26b8:	7194                	ld	a3,32(a1)
    26ba:	758c                	ld	a1,40(a1)
    26bc:	f832                	sd	a2,48(sp)
    26be:	f43a                	sd	a4,40(sp)
    26c0:	6d10                	ld	a2,24(a0)
    26c2:	6918                	ld	a4,16(a0)
    26c4:	651c                	ld	a5,8(a0)
    26c6:	6108                	ld	a0,0(a0)
    26c8:	f032                	sd	a2,32(sp)
    26ca:	ec3a                	sd	a4,24(sp)
    26cc:	e83e                	sd	a5,16(sp)
    26ce:	e42a                	sd	a0,8(sp)
    26d0:	0030                	addi	a2,sp,8
    26d2:	8536                	mv	a0,a3
    26d4:	00000097          	auipc	ra,0x0
    26d8:	00e080e7          	jalr	14(ra) # 26e2 <_ZN4core3fmt5write17h6dfe8a21eb788173E>
    26dc:	70e2                	ld	ra,56(sp)
    26de:	6121                	addi	sp,sp,64
    26e0:	8082                	ret

00000000000026e2 <_ZN4core3fmt5write17h6dfe8a21eb788173E>:
    26e2:	7135                	addi	sp,sp,-160
    26e4:	ed06                	sd	ra,152(sp)
    26e6:	e922                	sd	s0,144(sp)
    26e8:	e526                	sd	s1,136(sp)
    26ea:	e14a                	sd	s2,128(sp)
    26ec:	fcce                	sd	s3,120(sp)
    26ee:	f8d2                	sd	s4,112(sp)
    26f0:	f4d6                	sd	s5,104(sp)
    26f2:	f0da                	sd	s6,96(sp)
    26f4:	ecde                	sd	s7,88(sp)
    26f6:	e8e2                	sd	s8,80(sp)
    26f8:	e4e6                	sd	s9,72(sp)
    26fa:	8432                	mv	s0,a2
    26fc:	4605                	li	a2,1
    26fe:	1616                	slli	a2,a2,0x25
    2700:	fc32                	sd	a2,56(sp)
    2702:	460d                	li	a2,3
    2704:	04c10023          	sb	a2,64(sp)
    2708:	6804                	ld	s1,16(s0)
    270a:	e402                	sd	zero,8(sp)
    270c:	ec02                	sd	zero,24(sp)
    270e:	f42a                	sd	a0,40(sp)
    2710:	f82e                	sd	a1,48(sp)
    2712:	cce9                	beqz	s1,27ec <.LBB169_35+0x9e>
    2714:	6c10                	ld	a2,24(s0)
    2716:	00843983          	ld	s3,8(s0)
    271a:	00043903          	ld	s2,0(s0)
    271e:	8ace                	mv	s5,s3
    2720:	00c9e363          	bltu	s3,a2,2726 <_ZN4core3fmt5write17h6dfe8a21eb788173E+0x44>
    2724:	8ab2                	mv	s5,a2
    2726:	120a8163          	beqz	s5,2848 <.LBB169_35+0xfa>
    272a:	00093683          	ld	a3,0(s2)
    272e:	00893603          	ld	a2,8(s2)
    2732:	6d98                	ld	a4,24(a1)
    2734:	85b6                	mv	a1,a3
    2736:	9702                	jalr	a4
    2738:	12051663          	bnez	a0,2864 <.LBB169_35+0x116>
    273c:	02043c03          	ld	s8,32(s0)
    2740:	03048493          	addi	s1,s1,48
    2744:	01890c93          	addi	s9,s2,24
    2748:	00810a13          	addi	s4,sp,8
    274c:	4b09                	li	s6,2

000000000000274e <.LBB169_35>:
    274e:	fffffb97          	auipc	s7,0xfffff
    2752:	734b8b93          	addi	s7,s7,1844 # 1e82 <_ZN4core3ops8function6FnOnce9call_once17hb87d53624de904adE>
    2756:	8456                	mv	s0,s5
    2758:	ff84a503          	lw	a0,-8(s1)
    275c:	de2a                	sw	a0,60(sp)
    275e:	00048503          	lb	a0,0(s1)
    2762:	04a10023          	sb	a0,64(sp)
    2766:	ffc4a503          	lw	a0,-4(s1)
    276a:	dc2a                	sw	a0,56(sp)
    276c:	fe84b603          	ld	a2,-24(s1)
    2770:	ff04b503          	ld	a0,-16(s1)
    2774:	ca19                	beqz	a2,278a <.LBB169_35+0x3c>
    2776:	4581                	li	a1,0
    2778:	01660c63          	beq	a2,s6,2790 <.LBB169_35+0x42>
    277c:	0512                	slli	a0,a0,0x4
    277e:	9562                	add	a0,a0,s8
    2780:	650c                	ld	a1,8(a0)
    2782:	01759663          	bne	a1,s7,278e <.LBB169_35+0x40>
    2786:	6108                	ld	a0,0(a0)
    2788:	6108                	ld	a0,0(a0)
    278a:	4585                	li	a1,1
    278c:	a011                	j	2790 <.LBB169_35+0x42>
    278e:	4581                	li	a1,0
    2790:	e42e                	sd	a1,8(sp)
    2792:	e82a                	sd	a0,16(sp)
    2794:	fd84b603          	ld	a2,-40(s1)
    2798:	fe04b503          	ld	a0,-32(s1)
    279c:	ca19                	beqz	a2,27b2 <.LBB169_35+0x64>
    279e:	4581                	li	a1,0
    27a0:	01660c63          	beq	a2,s6,27b8 <.LBB169_35+0x6a>
    27a4:	0512                	slli	a0,a0,0x4
    27a6:	9562                	add	a0,a0,s8
    27a8:	650c                	ld	a1,8(a0)
    27aa:	01759663          	bne	a1,s7,27b6 <.LBB169_35+0x68>
    27ae:	6108                	ld	a0,0(a0)
    27b0:	6108                	ld	a0,0(a0)
    27b2:	4585                	li	a1,1
    27b4:	a011                	j	27b8 <.LBB169_35+0x6a>
    27b6:	4581                	li	a1,0
    27b8:	ec2e                	sd	a1,24(sp)
    27ba:	f02a                	sd	a0,32(sp)
    27bc:	fd04b503          	ld	a0,-48(s1)
    27c0:	0512                	slli	a0,a0,0x4
    27c2:	9562                	add	a0,a0,s8
    27c4:	6510                	ld	a2,8(a0)
    27c6:	6108                	ld	a0,0(a0)
    27c8:	85d2                	mv	a1,s4
    27ca:	9602                	jalr	a2
    27cc:	ed41                	bnez	a0,2864 <.LBB169_35+0x116>
    27ce:	147d                	addi	s0,s0,-1
    27d0:	cc2d                	beqz	s0,284a <.LBB169_35+0xfc>
    27d2:	76c2                	ld	a3,48(sp)
    27d4:	7522                	ld	a0,40(sp)
    27d6:	ff8cb583          	ld	a1,-8(s9)
    27da:	000cb603          	ld	a2,0(s9)
    27de:	6e94                	ld	a3,24(a3)
    27e0:	03848493          	addi	s1,s1,56
    27e4:	0cc1                	addi	s9,s9,16
    27e6:	9682                	jalr	a3
    27e8:	d925                	beqz	a0,2758 <.LBB169_35+0xa>
    27ea:	a8ad                	j	2864 <.LBB169_35+0x116>
    27ec:	7004                	ld	s1,32(s0)
    27ee:	7410                	ld	a2,40(s0)
    27f0:	00843983          	ld	s3,8(s0)
    27f4:	00043903          	ld	s2,0(s0)
    27f8:	8ace                	mv	s5,s3
    27fa:	00c9e363          	bltu	s3,a2,2800 <.LBB169_35+0xb2>
    27fe:	8ab2                	mv	s5,a2
    2800:	040a8463          	beqz	s5,2848 <.LBB169_35+0xfa>
    2804:	00093683          	ld	a3,0(s2)
    2808:	00893603          	ld	a2,8(s2)
    280c:	6d98                	ld	a4,24(a1)
    280e:	85b6                	mv	a1,a3
    2810:	9702                	jalr	a4
    2812:	e929                	bnez	a0,2864 <.LBB169_35+0x116>
    2814:	04a1                	addi	s1,s1,8
    2816:	01890b13          	addi	s6,s2,24
    281a:	00810a13          	addi	s4,sp,8
    281e:	8456                	mv	s0,s5
    2820:	6090                	ld	a2,0(s1)
    2822:	ff84b503          	ld	a0,-8(s1)
    2826:	85d2                	mv	a1,s4
    2828:	9602                	jalr	a2
    282a:	ed0d                	bnez	a0,2864 <.LBB169_35+0x116>
    282c:	147d                	addi	s0,s0,-1
    282e:	cc11                	beqz	s0,284a <.LBB169_35+0xfc>
    2830:	76c2                	ld	a3,48(sp)
    2832:	7522                	ld	a0,40(sp)
    2834:	ff8b3583          	ld	a1,-8(s6)
    2838:	000b3603          	ld	a2,0(s6)
    283c:	6e94                	ld	a3,24(a3)
    283e:	04c1                	addi	s1,s1,16
    2840:	0b41                	addi	s6,s6,16
    2842:	9682                	jalr	a3
    2844:	dd71                	beqz	a0,2820 <.LBB169_35+0xd2>
    2846:	a839                	j	2864 <.LBB169_35+0x116>
    2848:	4a81                	li	s5,0
    284a:	013aff63          	bgeu	s5,s3,2868 <.LBB169_35+0x11a>
    284e:	7522                	ld	a0,40(sp)
    2850:	76c2                	ld	a3,48(sp)
    2852:	004a9593          	slli	a1,s5,0x4
    2856:	00b90633          	add	a2,s2,a1
    285a:	620c                	ld	a1,0(a2)
    285c:	6610                	ld	a2,8(a2)
    285e:	6e94                	ld	a3,24(a3)
    2860:	9682                	jalr	a3
    2862:	c119                	beqz	a0,2868 <.LBB169_35+0x11a>
    2864:	4505                	li	a0,1
    2866:	a011                	j	286a <.LBB169_35+0x11c>
    2868:	4501                	li	a0,0
    286a:	6ca6                	ld	s9,72(sp)
    286c:	6c46                	ld	s8,80(sp)
    286e:	6be6                	ld	s7,88(sp)
    2870:	7b06                	ld	s6,96(sp)
    2872:	7aa6                	ld	s5,104(sp)
    2874:	7a46                	ld	s4,112(sp)
    2876:	79e6                	ld	s3,120(sp)
    2878:	690a                	ld	s2,128(sp)
    287a:	64aa                	ld	s1,136(sp)
    287c:	644a                	ld	s0,144(sp)
    287e:	60ea                	ld	ra,152(sp)
    2880:	610d                	addi	sp,sp,160
    2882:	8082                	ret

0000000000002884 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E>:
    2884:	7159                	addi	sp,sp,-112
    2886:	f486                	sd	ra,104(sp)
    2888:	f0a2                	sd	s0,96(sp)
    288a:	eca6                	sd	s1,88(sp)
    288c:	e8ca                	sd	s2,80(sp)
    288e:	e4ce                	sd	s3,72(sp)
    2890:	e0d2                	sd	s4,64(sp)
    2892:	fc56                	sd	s5,56(sp)
    2894:	f85a                	sd	s6,48(sp)
    2896:	f45e                	sd	s7,40(sp)
    2898:	f062                	sd	s8,32(sp)
    289a:	ec66                	sd	s9,24(sp)
    289c:	e86a                	sd	s10,16(sp)
    289e:	e46e                	sd	s11,8(sp)
    28a0:	89be                	mv	s3,a5
    28a2:	893a                	mv	s2,a4
    28a4:	8ab6                	mv	s5,a3
    28a6:	842a                	mv	s0,a0
    28a8:	c5b9                	beqz	a1,28f6 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x72>
    28aa:	03046503          	lwu	a0,48(s0)
    28ae:	00157593          	andi	a1,a0,1
    28b2:	00110a37          	lui	s4,0x110
    28b6:	c199                	beqz	a1,28bc <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x38>
    28b8:	02b00a13          	li	s4,43
    28bc:	01358cb3          	add	s9,a1,s3
    28c0:	00457593          	andi	a1,a0,4
    28c4:	c1b1                	beqz	a1,2908 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x84>
    28c6:	4581                	li	a1,0
    28c8:	020a8063          	beqz	s5,28e8 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x64>
    28cc:	86d6                	mv	a3,s5
    28ce:	8732                	mv	a4,a2
    28d0:	00074783          	lbu	a5,0(a4)
    28d4:	0705                	addi	a4,a4,1
    28d6:	0c07f793          	andi	a5,a5,192
    28da:	f8078793          	addi	a5,a5,-128
    28de:	00f037b3          	snez	a5,a5
    28e2:	16fd                	addi	a3,a3,-1
    28e4:	95be                	add	a1,a1,a5
    28e6:	f6ed                	bnez	a3,28d0 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x4c>
    28e8:	9cae                	add	s9,s9,a1
    28ea:	8b32                	mv	s6,a2
    28ec:	600c                	ld	a1,0(s0)
    28ee:	4d85                	li	s11,1
    28f0:	03b58163          	beq	a1,s11,2912 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x8e>
    28f4:	a0b1                	j	2940 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xbc>
    28f6:	03046503          	lwu	a0,48(s0)
    28fa:	00198c93          	addi	s9,s3,1
    28fe:	02d00a13          	li	s4,45
    2902:	00457593          	andi	a1,a0,4
    2906:	f1e1                	bnez	a1,28c6 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x42>
    2908:	4b01                	li	s6,0
    290a:	600c                	ld	a1,0(s0)
    290c:	4d85                	li	s11,1
    290e:	03b59963          	bne	a1,s11,2940 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xbc>
    2912:	00843d03          	ld	s10,8(s0)
    2916:	03acf563          	bgeu	s9,s10,2940 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xbc>
    291a:	8921                	andi	a0,a0,8
    291c:	ed3d                	bnez	a0,299a <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x116>
    291e:	03844503          	lbu	a0,56(s0)
    2922:	4585                	li	a1,1
    2924:	468d                	li	a3,3
    2926:	4605                	li	a2,1
    2928:	00d50363          	beq	a0,a3,292e <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xaa>
    292c:	862a                	mv	a2,a0
    292e:	8a0d                	andi	a2,a2,3
    2930:	419d0533          	sub	a0,s10,s9
    2934:	0ac5c763          	blt	a1,a2,29e2 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x15e>
    2938:	ea45                	bnez	a2,29e8 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x164>
    293a:	8c2a                	mv	s8,a0
    293c:	4501                	li	a0,0
    293e:	a0c9                	j	2a00 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x17c>
    2940:	8522                	mv	a0,s0
    2942:	85d2                	mv	a1,s4
    2944:	865a                	mv	a2,s6
    2946:	86d6                	mv	a3,s5
    2948:	00000097          	auipc	ra,0x0
    294c:	174080e7          	jalr	372(ra) # 2abc <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h223b8107861b7d1cE>
    2950:	c10d                	beqz	a0,2972 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xee>
    2952:	856e                	mv	a0,s11
    2954:	6da2                	ld	s11,8(sp)
    2956:	6d42                	ld	s10,16(sp)
    2958:	6ce2                	ld	s9,24(sp)
    295a:	7c02                	ld	s8,32(sp)
    295c:	7ba2                	ld	s7,40(sp)
    295e:	7b42                	ld	s6,48(sp)
    2960:	7ae2                	ld	s5,56(sp)
    2962:	6a06                	ld	s4,64(sp)
    2964:	69a6                	ld	s3,72(sp)
    2966:	6946                	ld	s2,80(sp)
    2968:	64e6                	ld	s1,88(sp)
    296a:	7406                	ld	s0,96(sp)
    296c:	70a6                	ld	ra,104(sp)
    296e:	6165                	addi	sp,sp,112
    2970:	8082                	ret
    2972:	740c                	ld	a1,40(s0)
    2974:	7008                	ld	a0,32(s0)
    2976:	6d9c                	ld	a5,24(a1)
    2978:	85ca                	mv	a1,s2
    297a:	864e                	mv	a2,s3
    297c:	6da2                	ld	s11,8(sp)
    297e:	6d42                	ld	s10,16(sp)
    2980:	6ce2                	ld	s9,24(sp)
    2982:	7c02                	ld	s8,32(sp)
    2984:	7ba2                	ld	s7,40(sp)
    2986:	7b42                	ld	s6,48(sp)
    2988:	7ae2                	ld	s5,56(sp)
    298a:	6a06                	ld	s4,64(sp)
    298c:	69a6                	ld	s3,72(sp)
    298e:	6946                	ld	s2,80(sp)
    2990:	64e6                	ld	s1,88(sp)
    2992:	7406                	ld	s0,96(sp)
    2994:	70a6                	ld	ra,104(sp)
    2996:	6165                	addi	sp,sp,112
    2998:	8782                	jr	a5
    299a:	03446b83          	lwu	s7,52(s0)
    299e:	03000513          	li	a0,48
    29a2:	03844c03          	lbu	s8,56(s0)
    29a6:	d848                	sw	a0,52(s0)
    29a8:	4d85                	li	s11,1
    29aa:	03b40c23          	sb	s11,56(s0)
    29ae:	8522                	mv	a0,s0
    29b0:	85d2                	mv	a1,s4
    29b2:	865a                	mv	a2,s6
    29b4:	86d6                	mv	a3,s5
    29b6:	00000097          	auipc	ra,0x0
    29ba:	106080e7          	jalr	262(ra) # 2abc <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h223b8107861b7d1cE>
    29be:	f951                	bnez	a0,2952 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xce>
    29c0:	03844503          	lbu	a0,56(s0)
    29c4:	4585                	li	a1,1
    29c6:	468d                	li	a3,3
    29c8:	4605                	li	a2,1
    29ca:	00d50363          	beq	a0,a3,29d0 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x14c>
    29ce:	862a                	mv	a2,a0
    29d0:	8a0d                	andi	a2,a2,3
    29d2:	419d0533          	sub	a0,s10,s9
    29d6:	00c5cb63          	blt	a1,a2,29ec <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x168>
    29da:	ee01                	bnez	a2,29f2 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x16e>
    29dc:	8aaa                	mv	s5,a0
    29de:	4501                	li	a0,0
    29e0:	a041                	j	2a60 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x1dc>
    29e2:	458d                	li	a1,3
    29e4:	00b61963          	bne	a2,a1,29f6 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x172>
    29e8:	4c01                	li	s8,0
    29ea:	a819                	j	2a00 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x17c>
    29ec:	458d                	li	a1,3
    29ee:	06b61463          	bne	a2,a1,2a56 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x1d2>
    29f2:	4a81                	li	s5,0
    29f4:	a0b5                	j	2a60 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x1dc>
    29f6:	00150593          	addi	a1,a0,1
    29fa:	8105                	srli	a0,a0,0x1
    29fc:	0015dc13          	srli	s8,a1,0x1
    2a00:	00150493          	addi	s1,a0,1
    2a04:	14fd                	addi	s1,s1,-1
    2a06:	c881                	beqz	s1,2a16 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x192>
    2a08:	740c                	ld	a1,40(s0)
    2a0a:	7008                	ld	a0,32(s0)
    2a0c:	7190                	ld	a2,32(a1)
    2a0e:	584c                	lw	a1,52(s0)
    2a10:	9602                	jalr	a2
    2a12:	d96d                	beqz	a0,2a04 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x180>
    2a14:	a085                	j	2a74 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x1f0>
    2a16:	03446b83          	lwu	s7,52(s0)
    2a1a:	8522                	mv	a0,s0
    2a1c:	85d2                	mv	a1,s4
    2a1e:	865a                	mv	a2,s6
    2a20:	86d6                	mv	a3,s5
    2a22:	00000097          	auipc	ra,0x0
    2a26:	09a080e7          	jalr	154(ra) # 2abc <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h223b8107861b7d1cE>
    2a2a:	4d85                	li	s11,1
    2a2c:	f11d                	bnez	a0,2952 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xce>
    2a2e:	740c                	ld	a1,40(s0)
    2a30:	7008                	ld	a0,32(s0)
    2a32:	6d94                	ld	a3,24(a1)
    2a34:	85ca                	mv	a1,s2
    2a36:	864e                	mv	a2,s3
    2a38:	9682                	jalr	a3
    2a3a:	fd01                	bnez	a0,2952 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xce>
    2a3c:	02043903          	ld	s2,32(s0)
    2a40:	7400                	ld	s0,40(s0)
    2a42:	001c0493          	addi	s1,s8,1
    2a46:	14fd                	addi	s1,s1,-1
    2a48:	c0b5                	beqz	s1,2aac <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x228>
    2a4a:	7010                	ld	a2,32(s0)
    2a4c:	854a                	mv	a0,s2
    2a4e:	85de                	mv	a1,s7
    2a50:	9602                	jalr	a2
    2a52:	d975                	beqz	a0,2a46 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x1c2>
    2a54:	bdfd                	j	2952 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xce>
    2a56:	00150593          	addi	a1,a0,1
    2a5a:	8105                	srli	a0,a0,0x1
    2a5c:	0015da93          	srli	s5,a1,0x1
    2a60:	00150493          	addi	s1,a0,1
    2a64:	14fd                	addi	s1,s1,-1
    2a66:	c889                	beqz	s1,2a78 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x1f4>
    2a68:	740c                	ld	a1,40(s0)
    2a6a:	7008                	ld	a0,32(s0)
    2a6c:	7190                	ld	a2,32(a1)
    2a6e:	584c                	lw	a1,52(s0)
    2a70:	9602                	jalr	a2
    2a72:	d96d                	beqz	a0,2a64 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x1e0>
    2a74:	4d85                	li	s11,1
    2a76:	bdf1                	j	2952 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xce>
    2a78:	740c                	ld	a1,40(s0)
    2a7a:	03446a03          	lwu	s4,52(s0)
    2a7e:	7008                	ld	a0,32(s0)
    2a80:	6d94                	ld	a3,24(a1)
    2a82:	85ca                	mv	a1,s2
    2a84:	864e                	mv	a2,s3
    2a86:	9682                	jalr	a3
    2a88:	4d85                	li	s11,1
    2a8a:	ec0514e3          	bnez	a0,2952 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xce>
    2a8e:	02043903          	ld	s2,32(s0)
    2a92:	02843983          	ld	s3,40(s0)
    2a96:	001a8493          	addi	s1,s5,1
    2a9a:	14fd                	addi	s1,s1,-1
    2a9c:	c891                	beqz	s1,2ab0 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x22c>
    2a9e:	0209b603          	ld	a2,32(s3)
    2aa2:	854a                	mv	a0,s2
    2aa4:	85d2                	mv	a1,s4
    2aa6:	9602                	jalr	a2
    2aa8:	d96d                	beqz	a0,2a9a <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0x216>
    2aaa:	b565                	j	2952 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xce>
    2aac:	4d81                	li	s11,0
    2aae:	b555                	j	2952 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xce>
    2ab0:	4d81                	li	s11,0
    2ab2:	03742a23          	sw	s7,52(s0)
    2ab6:	03840c23          	sb	s8,56(s0)
    2aba:	bd61                	j	2952 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E+0xce>

0000000000002abc <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h223b8107861b7d1cE>:
    2abc:	1101                	addi	sp,sp,-32
    2abe:	ec06                	sd	ra,24(sp)
    2ac0:	e822                	sd	s0,16(sp)
    2ac2:	e426                	sd	s1,8(sp)
    2ac4:	e04a                	sd	s2,0(sp)
    2ac6:	02059713          	slli	a4,a1,0x20
    2aca:	9301                	srli	a4,a4,0x20
    2acc:	001107b7          	lui	a5,0x110
    2ad0:	8936                	mv	s2,a3
    2ad2:	84b2                	mv	s1,a2
    2ad4:	842a                	mv	s0,a0
    2ad6:	00f70963          	beq	a4,a5,2ae8 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h223b8107861b7d1cE+0x2c>
    2ada:	7410                	ld	a2,40(s0)
    2adc:	7008                	ld	a0,32(s0)
    2ade:	7210                	ld	a2,32(a2)
    2ae0:	9602                	jalr	a2
    2ae2:	85aa                	mv	a1,a0
    2ae4:	4505                	li	a0,1
    2ae6:	ed91                	bnez	a1,2b02 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h223b8107861b7d1cE+0x46>
    2ae8:	cc81                	beqz	s1,2b00 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17h223b8107861b7d1cE+0x44>
    2aea:	740c                	ld	a1,40(s0)
    2aec:	7008                	ld	a0,32(s0)
    2aee:	6d9c                	ld	a5,24(a1)
    2af0:	85a6                	mv	a1,s1
    2af2:	864a                	mv	a2,s2
    2af4:	6902                	ld	s2,0(sp)
    2af6:	64a2                	ld	s1,8(sp)
    2af8:	6442                	ld	s0,16(sp)
    2afa:	60e2                	ld	ra,24(sp)
    2afc:	6105                	addi	sp,sp,32
    2afe:	8782                	jr	a5
    2b00:	4501                	li	a0,0
    2b02:	6902                	ld	s2,0(sp)
    2b04:	64a2                	ld	s1,8(sp)
    2b06:	6442                	ld	s0,16(sp)
    2b08:	60e2                	ld	ra,24(sp)
    2b0a:	6105                	addi	sp,sp,32
    2b0c:	8082                	ret

0000000000002b0e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E>:
    2b0e:	7139                	addi	sp,sp,-64
    2b10:	fc06                	sd	ra,56(sp)
    2b12:	f822                	sd	s0,48(sp)
    2b14:	f426                	sd	s1,40(sp)
    2b16:	f04a                	sd	s2,32(sp)
    2b18:	ec4e                	sd	s3,24(sp)
    2b1a:	e852                	sd	s4,16(sp)
    2b1c:	e456                	sd	s5,8(sp)
    2b1e:	e05a                	sd	s6,0(sp)
    2b20:	8b2a                	mv	s6,a0
    2b22:	6914                	ld	a3,16(a0)
    2b24:	6108                	ld	a0,0(a0)
    2b26:	89b2                	mv	s3,a2
    2b28:	892e                	mv	s2,a1
    2b2a:	fff68593          	addi	a1,a3,-1
    2b2e:	4605                	li	a2,1
    2b30:	0015b593          	seqz	a1,a1
    2b34:	00c51463          	bne	a0,a2,2b3c <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x2e>
    2b38:	e581                	bnez	a1,2b40 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x32>
    2b3a:	a215                	j	2c5e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x150>
    2b3c:	18058b63          	beqz	a1,2cd2 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1c4>
    2b40:	018b3683          	ld	a3,24(s6)
    2b44:	157d                	addi	a0,a0,-1
    2b46:	00153813          	seqz	a6,a0
    2b4a:	01390733          	add	a4,s2,s3
    2b4e:	4581                	li	a1,0
    2b50:	c2dd                	beqz	a3,2bf6 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xe8>
    2b52:	5e7d                	li	t3,-1
    2b54:	0e000393          	li	t2,224
    2b58:	0f000313          	li	t1,240
    2b5c:	001c08b7          	lui	a7,0x1c0
    2b60:	001102b7          	lui	t0,0x110
    2b64:	854a                	mv	a0,s2
    2b66:	a815                	j	2b9a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x8c>
    2b68:	00064403          	lbu	s0,0(a2)
    2b6c:	00160493          	addi	s1,a2,1
    2b70:	03f47f93          	andi	t6,s0,63
    2b74:	07ca                	slli	a5,a5,0x12
    2b76:	0117f7b3          	and	a5,a5,a7
    2b7a:	00cf1413          	slli	s0,t5,0xc
    2b7e:	006e9613          	slli	a2,t4,0x6
    2b82:	8fc1                	or	a5,a5,s0
    2b84:	8e5d                	or	a2,a2,a5
    2b86:	01f66633          	or	a2,a2,t6
    2b8a:	0c560863          	beq	a2,t0,2c5a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x14c>
    2b8e:	40a48533          	sub	a0,s1,a0
    2b92:	16fd                	addi	a3,a3,-1
    2b94:	95aa                	add	a1,a1,a0
    2b96:	8526                	mv	a0,s1
    2b98:	c2a5                	beqz	a3,2bf8 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xea>
    2b9a:	0ca70063          	beq	a4,a0,2c5a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x14c>
    2b9e:	00050603          	lb	a2,0(a0)
    2ba2:	00150493          	addi	s1,a0,1
    2ba6:	fece44e3          	blt	t3,a2,2b8e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x80>
    2baa:	0ff67793          	andi	a5,a2,255
    2bae:	00e48c63          	beq	s1,a4,2bc6 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xb8>
    2bb2:	00154603          	lbu	a2,1(a0)
    2bb6:	00250493          	addi	s1,a0,2
    2bba:	03f67f13          	andi	t5,a2,63
    2bbe:	8626                	mv	a2,s1
    2bc0:	fc77e7e3          	bltu	a5,t2,2b8e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x80>
    2bc4:	a029                	j	2bce <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xc0>
    2bc6:	4f01                	li	t5,0
    2bc8:	863a                	mv	a2,a4
    2bca:	fc77e2e3          	bltu	a5,t2,2b8e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x80>
    2bce:	00e60c63          	beq	a2,a4,2be6 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xd8>
    2bd2:	00064403          	lbu	s0,0(a2)
    2bd6:	00160493          	addi	s1,a2,1
    2bda:	03f47e93          	andi	t4,s0,63
    2bde:	8626                	mv	a2,s1
    2be0:	fa67e7e3          	bltu	a5,t1,2b8e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x80>
    2be4:	a029                	j	2bee <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xe0>
    2be6:	4e81                	li	t4,0
    2be8:	863a                	mv	a2,a4
    2bea:	fa67e2e3          	bltu	a5,t1,2b8e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x80>
    2bee:	f6e61de3          	bne	a2,a4,2b68 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x5a>
    2bf2:	4f81                	li	t6,0
    2bf4:	b741                	j	2b74 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x66>
    2bf6:	84ca                	mv	s1,s2
    2bf8:	06970163          	beq	a4,s1,2c5a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x14c>
    2bfc:	00048503          	lb	a0,0(s1)
    2c00:	567d                	li	a2,-1
    2c02:	02a65763          	bge	a2,a0,2c30 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x122>
    2c06:	0015b513          	seqz	a0,a1
    2c0a:	0135c633          	xor	a2,a1,s3
    2c0e:	00163613          	seqz	a2,a2
    2c12:	8d51                	or	a0,a0,a2
    2c14:	e919                	bnez	a0,2c2a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x11c>
    2c16:	0335fe63          	bgeu	a1,s3,2c52 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x144>
    2c1a:	00b90533          	add	a0,s2,a1
    2c1e:	00050503          	lb	a0,0(a0)
    2c22:	fc000613          	li	a2,-64
    2c26:	02c54663          	blt	a0,a2,2c52 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x144>
    2c2a:	854a                	mv	a0,s2
    2c2c:	e50d                	bnez	a0,2c56 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x148>
    2c2e:	a035                	j	2c5a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x14c>
    2c30:	00148613          	addi	a2,s1,1
    2c34:	0ff57513          	andi	a0,a0,255
    2c38:	12e60d63          	beq	a2,a4,2d72 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x264>
    2c3c:	0014c683          	lbu	a3,1(s1)
    2c40:	00248613          	addi	a2,s1,2
    2c44:	03f6f693          	andi	a3,a3,63
    2c48:	0e000793          	li	a5,224
    2c4c:	12f57963          	bgeu	a0,a5,2d7e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x270>
    2c50:	bf5d                	j	2c06 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xf8>
    2c52:	4501                	li	a0,0
    2c54:	c119                	beqz	a0,2c5a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x14c>
    2c56:	892a                	mv	s2,a0
    2c58:	89ae                	mv	s3,a1
    2c5a:	06080c63          	beqz	a6,2cd2 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1c4>
    2c5e:	04098563          	beqz	s3,2ca8 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x19a>
    2c62:	4581                	li	a1,0
    2c64:	854e                	mv	a0,s3
    2c66:	864a                	mv	a2,s2
    2c68:	00064683          	lbu	a3,0(a2)
    2c6c:	0605                	addi	a2,a2,1
    2c6e:	0c06f693          	andi	a3,a3,192
    2c72:	f8068693          	addi	a3,a3,-128
    2c76:	00d036b3          	snez	a3,a3
    2c7a:	157d                	addi	a0,a0,-1
    2c7c:	95b6                	add	a1,a1,a3
    2c7e:	f56d                	bnez	a0,2c68 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x15a>
    2c80:	008b3503          	ld	a0,8(s6)
    2c84:	04a5f763          	bgeu	a1,a0,2cd2 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1c4>
    2c88:	4581                	li	a1,0
    2c8a:	864e                	mv	a2,s3
    2c8c:	86ca                	mv	a3,s2
    2c8e:	0006c703          	lbu	a4,0(a3)
    2c92:	0685                	addi	a3,a3,1
    2c94:	0c077713          	andi	a4,a4,192
    2c98:	f8070713          	addi	a4,a4,-128
    2c9c:	00e03733          	snez	a4,a4
    2ca0:	167d                	addi	a2,a2,-1
    2ca2:	95ba                	add	a1,a1,a4
    2ca4:	f66d                	bnez	a2,2c8e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x180>
    2ca6:	a029                	j	2cb0 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1a2>
    2ca8:	008b3503          	ld	a0,8(s6)
    2cac:	4581                	li	a1,0
    2cae:	c115                	beqz	a0,2cd2 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1c4>
    2cb0:	038b4603          	lbu	a2,56(s6)
    2cb4:	470d                	li	a4,3
    2cb6:	4681                	li	a3,0
    2cb8:	00e60363          	beq	a2,a4,2cbe <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1b0>
    2cbc:	86b2                	mv	a3,a2
    2cbe:	0036f613          	andi	a2,a3,3
    2cc2:	4685                	li	a3,1
    2cc4:	8d0d                	sub	a0,a0,a1
    2cc6:	02c6c763          	blt	a3,a2,2cf4 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1e6>
    2cca:	ea05                	bnez	a2,2cfa <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1ec>
    2ccc:	8aaa                	mv	s5,a0
    2cce:	4501                	li	a0,0
    2cd0:	a825                	j	2d08 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1fa>
    2cd2:	028b3583          	ld	a1,40(s6)
    2cd6:	020b3503          	ld	a0,32(s6)
    2cda:	6d9c                	ld	a5,24(a1)
    2cdc:	85ca                	mv	a1,s2
    2cde:	864e                	mv	a2,s3
    2ce0:	6b02                	ld	s6,0(sp)
    2ce2:	6aa2                	ld	s5,8(sp)
    2ce4:	6a42                	ld	s4,16(sp)
    2ce6:	69e2                	ld	s3,24(sp)
    2ce8:	7902                	ld	s2,32(sp)
    2cea:	74a2                	ld	s1,40(sp)
    2cec:	7442                	ld	s0,48(sp)
    2cee:	70e2                	ld	ra,56(sp)
    2cf0:	6121                	addi	sp,sp,64
    2cf2:	8782                	jr	a5
    2cf4:	458d                	li	a1,3
    2cf6:	00b61463          	bne	a2,a1,2cfe <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1f0>
    2cfa:	4a81                	li	s5,0
    2cfc:	a031                	j	2d08 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1fa>
    2cfe:	00150593          	addi	a1,a0,1
    2d02:	8105                	srli	a0,a0,0x1
    2d04:	0015da93          	srli	s5,a1,0x1
    2d08:	00150493          	addi	s1,a0,1
    2d0c:	14fd                	addi	s1,s1,-1
    2d0e:	cc81                	beqz	s1,2d26 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x218>
    2d10:	028b3583          	ld	a1,40(s6)
    2d14:	020b3503          	ld	a0,32(s6)
    2d18:	7190                	ld	a2,32(a1)
    2d1a:	034b2583          	lw	a1,52(s6)
    2d1e:	9602                	jalr	a2
    2d20:	d575                	beqz	a0,2d0c <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x1fe>
    2d22:	4905                	li	s2,1
    2d24:	a825                	j	2d5c <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x24e>
    2d26:	028b3583          	ld	a1,40(s6)
    2d2a:	034b6a03          	lwu	s4,52(s6)
    2d2e:	020b3503          	ld	a0,32(s6)
    2d32:	6d94                	ld	a3,24(a1)
    2d34:	85ca                	mv	a1,s2
    2d36:	864e                	mv	a2,s3
    2d38:	9682                	jalr	a3
    2d3a:	4905                	li	s2,1
    2d3c:	e105                	bnez	a0,2d5c <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x24e>
    2d3e:	020b3983          	ld	s3,32(s6)
    2d42:	028b3403          	ld	s0,40(s6)
    2d46:	001a8493          	addi	s1,s5,1
    2d4a:	14fd                	addi	s1,s1,-1
    2d4c:	c499                	beqz	s1,2d5a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x24c>
    2d4e:	7010                	ld	a2,32(s0)
    2d50:	854e                	mv	a0,s3
    2d52:	85d2                	mv	a1,s4
    2d54:	9602                	jalr	a2
    2d56:	d975                	beqz	a0,2d4a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x23c>
    2d58:	a011                	j	2d5c <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x24e>
    2d5a:	4901                	li	s2,0
    2d5c:	854a                	mv	a0,s2
    2d5e:	6b02                	ld	s6,0(sp)
    2d60:	6aa2                	ld	s5,8(sp)
    2d62:	6a42                	ld	s4,16(sp)
    2d64:	69e2                	ld	s3,24(sp)
    2d66:	7902                	ld	s2,32(sp)
    2d68:	74a2                	ld	s1,40(sp)
    2d6a:	7442                	ld	s0,48(sp)
    2d6c:	70e2                	ld	ra,56(sp)
    2d6e:	6121                	addi	sp,sp,64
    2d70:	8082                	ret
    2d72:	4681                	li	a3,0
    2d74:	863a                	mv	a2,a4
    2d76:	0e000793          	li	a5,224
    2d7a:	e8f566e3          	bltu	a0,a5,2c06 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xf8>
    2d7e:	00e60d63          	beq	a2,a4,2d98 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x28a>
    2d82:	00064483          	lbu	s1,0(a2)
    2d86:	00160793          	addi	a5,a2,1
    2d8a:	03f4f613          	andi	a2,s1,63
    2d8e:	0f000493          	li	s1,240
    2d92:	00957963          	bgeu	a0,s1,2da4 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x296>
    2d96:	bd85                	j	2c06 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xf8>
    2d98:	4601                	li	a2,0
    2d9a:	87ba                	mv	a5,a4
    2d9c:	0f000493          	li	s1,240
    2da0:	e69563e3          	bltu	a0,s1,2c06 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xf8>
    2da4:	00e78763          	beq	a5,a4,2db2 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x2a4>
    2da8:	0007c703          	lbu	a4,0(a5) # 110000 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x100e90>
    2dac:	03f77713          	andi	a4,a4,63
    2db0:	a011                	j	2db4 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x2a6>
    2db2:	4701                	li	a4,0
    2db4:	054a                	slli	a0,a0,0x12
    2db6:	001c07b7          	lui	a5,0x1c0
    2dba:	8d7d                	and	a0,a0,a5
    2dbc:	06b2                	slli	a3,a3,0xc
    2dbe:	061a                	slli	a2,a2,0x6
    2dc0:	8d55                	or	a0,a0,a3
    2dc2:	8d51                	or	a0,a0,a2
    2dc4:	8d59                	or	a0,a0,a4
    2dc6:	00110637          	lui	a2,0x110
    2dca:	e8c508e3          	beq	a0,a2,2c5a <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0x14c>
    2dce:	bd25                	j	2c06 <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E+0xf8>

0000000000002dd0 <_ZN4core3fmt9Formatter15debug_lower_hex17hdec886692a7b892cE>:
    2dd0:	03054503          	lbu	a0,48(a0)
    2dd4:	8941                	andi	a0,a0,16
    2dd6:	8111                	srli	a0,a0,0x4
    2dd8:	8082                	ret

0000000000002dda <_ZN4core3fmt9Formatter15debug_upper_hex17hfaedba9b5105966cE>:
    2dda:	03054503          	lbu	a0,48(a0)
    2dde:	02057513          	andi	a0,a0,32
    2de2:	8115                	srli	a0,a0,0x5
    2de4:	8082                	ret

0000000000002de6 <_ZN4core3fmt9Formatter11debug_tuple17h5b19d3349176789bE>:
    2de6:	1101                	addi	sp,sp,-32
    2de8:	ec06                	sd	ra,24(sp)
    2dea:	e822                	sd	s0,16(sp)
    2dec:	e426                	sd	s1,8(sp)
    2dee:	e04a                	sd	s2,0(sp)
    2df0:	842e                	mv	s0,a1
    2df2:	7598                	ld	a4,40(a1)
    2df4:	718c                	ld	a1,32(a1)
    2df6:	6f18                	ld	a4,24(a4)
    2df8:	8936                	mv	s2,a3
    2dfa:	84aa                	mv	s1,a0
    2dfc:	852e                	mv	a0,a1
    2dfe:	85b2                	mv	a1,a2
    2e00:	8636                	mv	a2,a3
    2e02:	9702                	jalr	a4
    2e04:	00193593          	seqz	a1,s2
    2e08:	e080                	sd	s0,0(s1)
    2e0a:	00a48823          	sb	a0,16(s1)
    2e0e:	0004b423          	sd	zero,8(s1)
    2e12:	00b488a3          	sb	a1,17(s1)
    2e16:	6902                	ld	s2,0(sp)
    2e18:	64a2                	ld	s1,8(sp)
    2e1a:	6442                	ld	s0,16(sp)
    2e1c:	60e2                	ld	ra,24(sp)
    2e1e:	6105                	addi	sp,sp,32
    2e20:	8082                	ret

0000000000002e22 <_ZN4core3fmt9Formatter10debug_list17h14691806d5f8258eE>:
    2e22:	1141                	addi	sp,sp,-16
    2e24:	e406                	sd	ra,8(sp)
    2e26:	e022                	sd	s0,0(sp)
    2e28:	842a                	mv	s0,a0
    2e2a:	750c                	ld	a1,40(a0)
    2e2c:	7108                	ld	a0,32(a0)
    2e2e:	6d94                	ld	a3,24(a1)

0000000000002e30 <.LBB190_1>:
    2e30:	00003597          	auipc	a1,0x3
    2e34:	c0258593          	addi	a1,a1,-1022 # 5a32 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.212>
    2e38:	4605                	li	a2,1
    2e3a:	9682                	jalr	a3
    2e3c:	85aa                	mv	a1,a0
    2e3e:	8522                	mv	a0,s0
    2e40:	6402                	ld	s0,0(sp)
    2e42:	60a2                	ld	ra,8(sp)
    2e44:	0141                	addi	sp,sp,16
    2e46:	8082                	ret

0000000000002e48 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E>:
    2e48:	7159                	addi	sp,sp,-112
    2e4a:	f486                	sd	ra,104(sp)
    2e4c:	f0a2                	sd	s0,96(sp)
    2e4e:	eca6                	sd	s1,88(sp)
    2e50:	e8ca                	sd	s2,80(sp)
    2e52:	e4ce                	sd	s3,72(sp)
    2e54:	e0d2                	sd	s4,64(sp)
    2e56:	fc56                	sd	s5,56(sp)
    2e58:	f85a                	sd	s6,48(sp)
    2e5a:	f45e                	sd	s7,40(sp)
    2e5c:	f062                	sd	s8,32(sp)
    2e5e:	ec66                	sd	s9,24(sp)
    2e60:	e86a                	sd	s10,16(sp)
    2e62:	e46e                	sd	s11,8(sp)
    2e64:	842e                	mv	s0,a1
    2e66:	758c                	ld	a1,40(a1)
    2e68:	7010                	ld	a2,32(s0)
    2e6a:	7194                	ld	a3,32(a1)
    2e6c:	84aa                	mv	s1,a0
    2e6e:	02700593          	li	a1,39
    2e72:	8532                	mv	a0,a2
    2e74:	9682                	jalr	a3
    2e76:	c10d                	beqz	a0,2e98 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x50>
    2e78:	4505                	li	a0,1
    2e7a:	6da2                	ld	s11,8(sp)
    2e7c:	6d42                	ld	s10,16(sp)
    2e7e:	6ce2                	ld	s9,24(sp)
    2e80:	7c02                	ld	s8,32(sp)
    2e82:	7ba2                	ld	s7,40(sp)
    2e84:	7b42                	ld	s6,48(sp)
    2e86:	7ae2                	ld	s5,56(sp)
    2e88:	6a06                	ld	s4,64(sp)
    2e8a:	69a6                	ld	s3,72(sp)
    2e8c:	6946                	ld	s2,80(sp)
    2e8e:	64e6                	ld	s1,88(sp)
    2e90:	7406                	ld	s0,96(sp)
    2e92:	70a6                	ld	ra,104(sp)
    2e94:	6165                	addi	sp,sp,112
    2e96:	8082                	ret
    2e98:	0004ea83          	lwu	s5,0(s1)
    2e9c:	02100513          	li	a0,33
    2ea0:	4489                	li	s1,2
    2ea2:	01554e63          	blt	a0,s5,2ebe <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x76>
    2ea6:	4525                	li	a0,9
    2ea8:	04aa8763          	beq	s5,a0,2ef6 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xae>
    2eac:	4529                	li	a0,10
    2eae:	04aa8763          	beq	s5,a0,2efc <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xb4>
    2eb2:	4535                	li	a0,13
    2eb4:	02aa9163          	bne	s5,a0,2ed6 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x8e>
    2eb8:	07200a93          	li	s5,114
    2ebc:	a091                	j	2f00 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xb8>
    2ebe:	02200513          	li	a0,34
    2ec2:	02aa8f63          	beq	s5,a0,2f00 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xb8>
    2ec6:	02700513          	li	a0,39
    2eca:	02aa8b63          	beq	s5,a0,2f00 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xb8>
    2ece:	05c00513          	li	a0,92
    2ed2:	02aa8763          	beq	s5,a0,2f00 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xb8>
    2ed6:	8556                	mv	a0,s5
    2ed8:	00001097          	auipc	ra,0x1
    2edc:	fb4080e7          	jalr	-76(ra) # 3e8c <_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h2d824475a5ff264fE>
    2ee0:	10051b63          	bnez	a0,2ff6 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x1ae>
    2ee4:	8556                	mv	a0,s5
    2ee6:	00000097          	auipc	ra,0x0
    2eea:	678080e7          	jalr	1656(ra) # 355e <_ZN4core7unicode9printable12is_printable17ha1b974ef4cc7a0dfE>
    2eee:	10050463          	beqz	a0,2ff6 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x1ae>
    2ef2:	4485                	li	s1,1
    2ef4:	a031                	j	2f00 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xb8>
    2ef6:	07400a93          	li	s5,116
    2efa:	a019                	j	2f00 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xb8>
    2efc:	06e00a93          	li	s5,110
    2f00:	4a09                	li	s4,2
    2f02:	4b05                	li	s6,1
    2f04:	f0100513          	li	a0,-255
    2f08:	1502                	slli	a0,a0,0x20
    2f0a:	fff50b93          	addi	s7,a0,-1
    2f0e:	4c29                	li	s8,10
    2f10:	020b1c93          	slli	s9,s6,0x20
    2f14:	4d0d                	li	s10,3
    2f16:	021b1d93          	slli	s11,s6,0x21
    2f1a:	4911                	li	s2,4
    2f1c:	a039                	j	2f2a <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xe2>
    2f1e:	4485                	li	s1,1
    2f20:	7410                	ld	a2,40(s0)
    2f22:	7008                	ld	a0,32(s0)
    2f24:	7210                	ld	a2,32(a2)
    2f26:	9602                	jalr	a2
    2f28:	f921                	bnez	a0,2e78 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x30>
    2f2a:	02049513          	slli	a0,s1,0x20
    2f2e:	9101                	srli	a0,a0,0x20
    2f30:	00ab4663          	blt	s6,a0,2f3c <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xf4>
    2f34:	cd49                	beqz	a0,2fce <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x186>
    2f36:	4481                	li	s1,0
    2f38:	85d6                	mv	a1,s5
    2f3a:	b7dd                	j	2f20 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xd8>
    2f3c:	05c00593          	li	a1,92
    2f40:	fd450fe3          	beq	a0,s4,2f1e <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xd6>
    2f44:	0209d513          	srli	a0,s3,0x20
    2f48:	0ff57513          	andi	a0,a0,255
    2f4c:	00aa4b63          	blt	s4,a0,2f62 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x11a>
    2f50:	cd3d                	beqz	a0,2fce <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x186>
    2f52:	03651663          	bne	a0,s6,2f7e <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x136>
    2f56:	0179f9b3          	and	s3,s3,s7
    2f5a:	448d                	li	s1,3
    2f5c:	07d00593          	li	a1,125
    2f60:	b7c1                	j	2f20 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xd8>
    2f62:	05a50063          	beq	a0,s10,2fa2 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x15a>
    2f66:	05251663          	bne	a0,s2,2fb2 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x16a>
    2f6a:	0179f533          	and	a0,s3,s7
    2f6e:	448d                	li	s1,3
    2f70:	02049593          	slli	a1,s1,0x20
    2f74:	00b569b3          	or	s3,a0,a1
    2f78:	07500593          	li	a1,117
    2f7c:	b755                	j	2f20 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xd8>
    2f7e:	6602                	ld	a2,0(sp)
    2f80:	00261513          	slli	a0,a2,0x2
    2f84:	00a9d53b          	srlw	a0,s3,a0
    2f88:	893d                	andi	a0,a0,15
    2f8a:	03000593          	li	a1,48
    2f8e:	01856463          	bltu	a0,s8,2f96 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x14e>
    2f92:	05700593          	li	a1,87
    2f96:	95aa                	add	a1,a1,a0
    2f98:	c60d                	beqz	a2,2fc2 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0x17a>
    2f9a:	167d                	addi	a2,a2,-1
    2f9c:	e032                	sd	a2,0(sp)
    2f9e:	448d                	li	s1,3
    2fa0:	b741                	j	2f20 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xd8>
    2fa2:	0179f533          	and	a0,s3,s7
    2fa6:	01b569b3          	or	s3,a0,s11
    2faa:	448d                	li	s1,3
    2fac:	07b00593          	li	a1,123
    2fb0:	bf85                	j	2f20 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xd8>
    2fb2:	0179f533          	and	a0,s3,s7
    2fb6:	022b1613          	slli	a2,s6,0x22
    2fba:	00c569b3          	or	s3,a0,a2
    2fbe:	448d                	li	s1,3
    2fc0:	b785                	j	2f20 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xd8>
    2fc2:	0179f533          	and	a0,s3,s7
    2fc6:	019569b3          	or	s3,a0,s9
    2fca:	448d                	li	s1,3
    2fcc:	bf91                	j	2f20 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xd8>
    2fce:	740c                	ld	a1,40(s0)
    2fd0:	7008                	ld	a0,32(s0)
    2fd2:	719c                	ld	a5,32(a1)
    2fd4:	02700593          	li	a1,39
    2fd8:	6da2                	ld	s11,8(sp)
    2fda:	6d42                	ld	s10,16(sp)
    2fdc:	6ce2                	ld	s9,24(sp)
    2fde:	7c02                	ld	s8,32(sp)
    2fe0:	7ba2                	ld	s7,40(sp)
    2fe2:	7b42                	ld	s6,48(sp)
    2fe4:	7ae2                	ld	s5,56(sp)
    2fe6:	6a06                	ld	s4,64(sp)
    2fe8:	69a6                	ld	s3,72(sp)
    2fea:	6946                	ld	s2,80(sp)
    2fec:	64e6                	ld	s1,88(sp)
    2fee:	7406                	ld	s0,96(sp)
    2ff0:	70a6                	ld	ra,104(sp)
    2ff2:	6165                	addi	sp,sp,112
    2ff4:	8782                	jr	a5
    2ff6:	001ae513          	ori	a0,s5,1
    2ffa:	00155593          	srli	a1,a0,0x1
    2ffe:	8d4d                	or	a0,a0,a1
    3000:	00255593          	srli	a1,a0,0x2
    3004:	8d4d                	or	a0,a0,a1
    3006:	00455593          	srli	a1,a0,0x4
    300a:	8d4d                	or	a0,a0,a1
    300c:	00855593          	srli	a1,a0,0x8
    3010:	8d4d                	or	a0,a0,a1
    3012:	01055593          	srli	a1,a0,0x10
    3016:	8d4d                	or	a0,a0,a1
    3018:	02055593          	srli	a1,a0,0x20
    301c:	8d4d                	or	a0,a0,a1
    301e:	fff54513          	not	a0,a0
    3022:	00155593          	srli	a1,a0,0x1
    3026:	05555637          	lui	a2,0x5555
    302a:	5556061b          	addiw	a2,a2,1365
    302e:	0632                	slli	a2,a2,0xc
    3030:	55560613          	addi	a2,a2,1365 # 5555555 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x55463e5>
    3034:	0632                	slli	a2,a2,0xc
    3036:	55560613          	addi	a2,a2,1365
    303a:	0632                	slli	a2,a2,0xc
    303c:	55560613          	addi	a2,a2,1365
    3040:	8df1                	and	a1,a1,a2
    3042:	8d0d                	sub	a0,a0,a1
    3044:	033335b7          	lui	a1,0x3333
    3048:	3335859b          	addiw	a1,a1,819
    304c:	05b2                	slli	a1,a1,0xc
    304e:	33358593          	addi	a1,a1,819 # 3333333 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x33241c3>
    3052:	05b2                	slli	a1,a1,0xc
    3054:	33358593          	addi	a1,a1,819
    3058:	05b2                	slli	a1,a1,0xc
    305a:	33358593          	addi	a1,a1,819
    305e:	00b57633          	and	a2,a0,a1
    3062:	8109                	srli	a0,a0,0x2
    3064:	8d6d                	and	a0,a0,a1
    3066:	9532                	add	a0,a0,a2
    3068:	00455593          	srli	a1,a0,0x4
    306c:	952e                	add	a0,a0,a1
    306e:	00f0f5b7          	lui	a1,0xf0f
    3072:	0f15859b          	addiw	a1,a1,241
    3076:	05b2                	slli	a1,a1,0xc
    3078:	f0f58593          	addi	a1,a1,-241 # f0ef0f <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xeffd9f>
    307c:	05b2                	slli	a1,a1,0xc
    307e:	0f158593          	addi	a1,a1,241
    3082:	05b2                	slli	a1,a1,0xc
    3084:	f0f58593          	addi	a1,a1,-241
    3088:	8d6d                	and	a0,a0,a1
    308a:	010105b7          	lui	a1,0x1010
    308e:	1015859b          	addiw	a1,a1,257
    3092:	05c2                	slli	a1,a1,0x10
    3094:	10158593          	addi	a1,a1,257 # 1010101 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1000f91>
    3098:	05c2                	slli	a1,a1,0x10
    309a:	10158593          	addi	a1,a1,257
    309e:	02b50533          	mul	a0,a0,a1
    30a2:	9161                	srli	a0,a0,0x38
    30a4:	1501                	addi	a0,a0,-32
    30a6:	0025551b          	srliw	a0,a0,0x2
    30aa:	00754513          	xori	a0,a0,7
    30ae:	e02a                	sd	a0,0(sp)
    30b0:	4515                	li	a0,5
    30b2:	1502                	slli	a0,a0,0x20
    30b4:	00aae9b3          	or	s3,s5,a0
    30b8:	448d                	li	s1,3
    30ba:	b599                	j	2f00 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E+0xb8>

00000000000030bc <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E>:
    30bc:	1141                	addi	sp,sp,-16
    30be:	e406                	sd	ra,8(sp)
    30c0:	00758693          	addi	a3,a1,7
    30c4:	9ae1                	andi	a3,a3,-8
    30c6:	8e8d                	sub	a3,a3,a1
    30c8:	0ff57813          	andi	a6,a0,255
    30cc:	c68d                	beqz	a3,30f6 <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0x3a>
    30ce:	8532                	mv	a0,a2
    30d0:	00d66363          	bltu	a2,a3,30d6 <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0x1a>
    30d4:	8536                	mv	a0,a3
    30d6:	c105                	beqz	a0,30f6 <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0x3a>
    30d8:	4681                	li	a3,0
    30da:	00d58733          	add	a4,a1,a3
    30de:	00074703          	lbu	a4,0(a4)
    30e2:	0d070463          	beq	a4,a6,31aa <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0xee>
    30e6:	0685                	addi	a3,a3,1
    30e8:	fed519e3          	bne	a0,a3,30da <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0x1e>
    30ec:	ff060893          	addi	a7,a2,-16
    30f0:	00a8f663          	bgeu	a7,a0,30fc <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0x40>
    30f4:	a049                	j	3176 <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0xba>
    30f6:	4501                	li	a0,0
    30f8:	ff060893          	addi	a7,a2,-16
    30fc:	feff06b7          	lui	a3,0xfeff0
    3100:	eff6869b          	addiw	a3,a3,-257
    3104:	06c2                	slli	a3,a3,0x10
    3106:	eff68693          	addi	a3,a3,-257 # fffffffffefefeff <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xfffffffffefe0d8f>
    310a:	06c2                	slli	a3,a3,0x10
    310c:	eff68293          	addi	t0,a3,-257
    3110:	76c1                	lui	a3,0xffff0
    3112:	1016869b          	addiw	a3,a3,257
    3116:	06c2                	slli	a3,a3,0x10
    3118:	10168693          	addi	a3,a3,257 # ffffffffffff0101 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xfffffffffffe0f91>
    311c:	06c2                	slli	a3,a3,0x10
    311e:	10168693          	addi	a3,a3,257
    3122:	06be                	slli	a3,a3,0xf
    3124:	08068313          	addi	t1,a3,128
    3128:	010106b7          	lui	a3,0x1010
    312c:	1016869b          	addiw	a3,a3,257
    3130:	06c2                	slli	a3,a3,0x10
    3132:	10168693          	addi	a3,a3,257 # 1010101 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1000f91>
    3136:	06c2                	slli	a3,a3,0x10
    3138:	10168693          	addi	a3,a3,257
    313c:	02d803b3          	mul	t2,a6,a3
    3140:	00a587b3          	add	a5,a1,a0
    3144:	6398                	ld	a4,0(a5)
    3146:	679c                	ld	a5,8(a5)
    3148:	00774733          	xor	a4,a4,t2
    314c:	fff74693          	not	a3,a4
    3150:	9716                	add	a4,a4,t0
    3152:	0066f6b3          	and	a3,a3,t1
    3156:	8ef9                	and	a3,a3,a4
    3158:	0077c733          	xor	a4,a5,t2
    315c:	fff74793          	not	a5,a4
    3160:	9716                	add	a4,a4,t0
    3162:	0067f7b3          	and	a5,a5,t1
    3166:	8f7d                	and	a4,a4,a5
    3168:	8ed9                	or	a3,a3,a4
    316a:	e681                	bnez	a3,3172 <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0xb6>
    316c:	0541                	addi	a0,a0,16
    316e:	fca8f9e3          	bgeu	a7,a0,3140 <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0x84>
    3172:	02a66e63          	bltu	a2,a0,31ae <.LBB205_24>
    3176:	4681                	li	a3,0
    3178:	00c51463          	bne	a0,a2,3180 <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0xc4>
    317c:	4581                	li	a1,0
    317e:	a005                	j	319e <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0xe2>
    3180:	8e09                	sub	a2,a2,a0
    3182:	95aa                	add	a1,a1,a0
    3184:	00d58733          	add	a4,a1,a3
    3188:	00074703          	lbu	a4,0(a4)
    318c:	01070863          	beq	a4,a6,319c <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0xe0>
    3190:	0685                	addi	a3,a3,1
    3192:	fed619e3          	bne	a2,a3,3184 <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0xc8>
    3196:	4581                	li	a1,0
    3198:	86b2                	mv	a3,a2
    319a:	a011                	j	319e <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0xe2>
    319c:	4585                	li	a1,1
    319e:	96aa                	add	a3,a3,a0
    31a0:	852e                	mv	a0,a1
    31a2:	85b6                	mv	a1,a3
    31a4:	60a2                	ld	ra,8(sp)
    31a6:	0141                	addi	sp,sp,16
    31a8:	8082                	ret
    31aa:	4585                	li	a1,1
    31ac:	bfd5                	j	31a0 <_ZN4core5slice6memchr19memchr_general_case17h265457709e3069b0E+0xe4>

00000000000031ae <.LBB205_24>:
    31ae:	00003697          	auipc	a3,0x3
    31b2:	9ba68693          	addi	a3,a3,-1606 # 5b68 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.245>
    31b6:	85b2                	mv	a1,a2
    31b8:	8636                	mv	a2,a3
    31ba:	00000097          	auipc	ra,0x0
    31be:	00a080e7          	jalr	10(ra) # 31c4 <_ZN4core5slice5index26slice_start_index_len_fail17hf5a8e79169c741aaE>
	...

00000000000031c4 <_ZN4core5slice5index26slice_start_index_len_fail17hf5a8e79169c741aaE>:
    31c4:	7159                	addi	sp,sp,-112
    31c6:	f486                	sd	ra,104(sp)
    31c8:	e42a                	sd	a0,8(sp)
    31ca:	e82e                	sd	a1,16(sp)
    31cc:	0028                	addi	a0,sp,8
    31ce:	e4aa                	sd	a0,72(sp)

00000000000031d0 <.LBB207_1>:
    31d0:	00001517          	auipc	a0,0x1
    31d4:	ae850513          	addi	a0,a0,-1304 # 3cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>
    31d8:	e8aa                	sd	a0,80(sp)
    31da:	080c                	addi	a1,sp,16
    31dc:	ecae                	sd	a1,88(sp)
    31de:	f0aa                	sd	a0,96(sp)

00000000000031e0 <.LBB207_2>:
    31e0:	00003517          	auipc	a0,0x3
    31e4:	9d850513          	addi	a0,a0,-1576 # 5bb8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.250>
    31e8:	ec2a                	sd	a0,24(sp)
    31ea:	4509                	li	a0,2
    31ec:	f02a                	sd	a0,32(sp)
    31ee:	f402                	sd	zero,40(sp)
    31f0:	00ac                	addi	a1,sp,72
    31f2:	fc2e                	sd	a1,56(sp)
    31f4:	e0aa                	sd	a0,64(sp)
    31f6:	0828                	addi	a0,sp,24
    31f8:	85b2                	mv	a1,a2
    31fa:	fffff097          	auipc	ra,0xfffff
    31fe:	db4080e7          	jalr	-588(ra) # 1fae <_ZN4core9panicking9panic_fmt17ha96d96e8fa0883ecE>
	...

0000000000003204 <_ZN4core5slice5index24slice_end_index_len_fail17h0db61fbd8d9e0e45E>:
    3204:	7159                	addi	sp,sp,-112
    3206:	f486                	sd	ra,104(sp)
    3208:	e42a                	sd	a0,8(sp)
    320a:	e82e                	sd	a1,16(sp)
    320c:	0028                	addi	a0,sp,8
    320e:	e4aa                	sd	a0,72(sp)

0000000000003210 <.LBB208_1>:
    3210:	00001517          	auipc	a0,0x1
    3214:	aa850513          	addi	a0,a0,-1368 # 3cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>
    3218:	e8aa                	sd	a0,80(sp)
    321a:	080c                	addi	a1,sp,16
    321c:	ecae                	sd	a1,88(sp)
    321e:	f0aa                	sd	a0,96(sp)

0000000000003220 <.LBB208_2>:
    3220:	00003517          	auipc	a0,0x3
    3224:	9b850513          	addi	a0,a0,-1608 # 5bd8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.252>
    3228:	ec2a                	sd	a0,24(sp)
    322a:	4509                	li	a0,2
    322c:	f02a                	sd	a0,32(sp)
    322e:	f402                	sd	zero,40(sp)
    3230:	00ac                	addi	a1,sp,72
    3232:	fc2e                	sd	a1,56(sp)
    3234:	e0aa                	sd	a0,64(sp)
    3236:	0828                	addi	a0,sp,24
    3238:	85b2                	mv	a1,a2
    323a:	fffff097          	auipc	ra,0xfffff
    323e:	d74080e7          	jalr	-652(ra) # 1fae <_ZN4core9panicking9panic_fmt17ha96d96e8fa0883ecE>
	...

0000000000003244 <_ZN4core5slice5index22slice_index_order_fail17h2f93cb17ebf66956E>:
    3244:	7159                	addi	sp,sp,-112
    3246:	f486                	sd	ra,104(sp)
    3248:	e42a                	sd	a0,8(sp)
    324a:	e82e                	sd	a1,16(sp)
    324c:	0028                	addi	a0,sp,8
    324e:	e4aa                	sd	a0,72(sp)

0000000000003250 <.LBB209_1>:
    3250:	00001517          	auipc	a0,0x1
    3254:	a6850513          	addi	a0,a0,-1432 # 3cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>
    3258:	e8aa                	sd	a0,80(sp)
    325a:	080c                	addi	a1,sp,16
    325c:	ecae                	sd	a1,88(sp)
    325e:	f0aa                	sd	a0,96(sp)

0000000000003260 <.LBB209_2>:
    3260:	00003517          	auipc	a0,0x3
    3264:	9c050513          	addi	a0,a0,-1600 # 5c20 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.255>
    3268:	ec2a                	sd	a0,24(sp)
    326a:	4509                	li	a0,2
    326c:	f02a                	sd	a0,32(sp)
    326e:	f402                	sd	zero,40(sp)
    3270:	00ac                	addi	a1,sp,72
    3272:	fc2e                	sd	a1,56(sp)
    3274:	e0aa                	sd	a0,64(sp)
    3276:	0828                	addi	a0,sp,24
    3278:	85b2                	mv	a1,a2
    327a:	fffff097          	auipc	ra,0xfffff
    327e:	d34080e7          	jalr	-716(ra) # 1fae <_ZN4core9panicking9panic_fmt17ha96d96e8fa0883ecE>
	...

0000000000003284 <_ZN4core3str16slice_error_fail17h4f668c8ce31695ebE>:
    3284:	7115                	addi	sp,sp,-224
    3286:	ed86                	sd	ra,216(sp)
    3288:	e432                	sd	a2,8(sp)
    328a:	e836                	sd	a3,16(sp)
    328c:	10100793          	li	a5,257
    3290:	4885                	li	a7,1
    3292:	882e                	mv	a6,a1
    3294:	04f5e263          	bltu	a1,a5,32d8 <_ZN4core3str16slice_error_fail17h4f668c8ce31695ebE+0x54>
    3298:	4381                	li	t2,0
    329a:	f0158893          	addi	a7,a1,-255
    329e:	10050293          	addi	t0,a0,256
    32a2:	fbf00313          	li	t1,-65
    32a6:	10038813          	addi	a6,t2,256
    32aa:	00b87863          	bgeu	a6,a1,32ba <_ZN4core3str16slice_error_fail17h4f668c8ce31695ebE+0x36>
    32ae:	007287b3          	add	a5,t0,t2
    32b2:	00078783          	lb	a5,0(a5) # 1c0000 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1b0e90>
    32b6:	02f34b63          	blt	t1,a5,32ec <.LBB232_49+0xc>
    32ba:	0ff38793          	addi	a5,t2,255
    32be:	00f03833          	snez	a6,a5
    32c2:	0078c7b3          	xor	a5,a7,t2
    32c6:	00f037b3          	snez	a5,a5
    32ca:	00f877b3          	and	a5,a6,a5
    32ce:	13fd                	addi	t2,t2,-1
    32d0:	fbf9                	bnez	a5,32a6 <_ZN4core3str16slice_error_fail17h4f668c8ce31695ebE+0x22>
    32d2:	4881                	li	a7,0
    32d4:	10038813          	addi	a6,t2,256
    32d8:	ec2a                	sd	a0,24(sp)
    32da:	f042                	sd	a6,32(sp)
    32dc:	00089d63          	bnez	a7,32f6 <.LBB232_49+0x16>

00000000000032e0 <.LBB232_49>:
    32e0:	00003797          	auipc	a5,0x3
    32e4:	96878793          	addi	a5,a5,-1688 # 5c48 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.313>
    32e8:	4815                	li	a6,5
    32ea:	a819                	j	3300 <.LBB232_50+0x8>
    32ec:	4881                	li	a7,0
    32ee:	ec2a                	sd	a0,24(sp)
    32f0:	f042                	sd	a6,32(sp)
    32f2:	fe0887e3          	beqz	a7,32e0 <.LBB232_49>
    32f6:	4801                	li	a6,0

00000000000032f8 <.LBB232_50>:
    32f8:	00002797          	auipc	a5,0x2
    32fc:	4e878793          	addi	a5,a5,1256 # 57e0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.134>
    3300:	f43e                	sd	a5,40(sp)
    3302:	00c5b8b3          	sltu	a7,a1,a2
    3306:	0018c293          	xori	t0,a7,1
    330a:	00d5b7b3          	sltu	a5,a1,a3
    330e:	0017c793          	xori	a5,a5,1
    3312:	00f2f7b3          	and	a5,t0,a5
    3316:	f842                	sd	a6,48(sp)
    3318:	ef8d                	bnez	a5,3352 <.LBB232_53+0xe>
    331a:	00089363          	bnez	a7,3320 <.LBB232_50+0x28>
    331e:	8636                	mv	a2,a3
    3320:	e4b2                	sd	a2,72(sp)
    3322:	00a8                	addi	a0,sp,72
    3324:	e52a                	sd	a0,136(sp)

0000000000003326 <.LBB232_51>:
    3326:	00001517          	auipc	a0,0x1
    332a:	99250513          	addi	a0,a0,-1646 # 3cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>
    332e:	e92a                	sd	a0,144(sp)
    3330:	0828                	addi	a0,sp,24
    3332:	ed2a                	sd	a0,152(sp)

0000000000003334 <.LBB232_52>:
    3334:	00001517          	auipc	a0,0x1
    3338:	b4850513          	addi	a0,a0,-1208 # 3e7c <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hc6a9db97781cac35E>
    333c:	f12a                	sd	a0,160(sp)
    333e:	102c                	addi	a1,sp,40
    3340:	f52e                	sd	a1,168(sp)
    3342:	f92a                	sd	a0,176(sp)

0000000000003344 <.LBB232_53>:
    3344:	00003517          	auipc	a0,0x3
    3348:	92c50513          	addi	a0,a0,-1748 # 5c70 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.316>
    334c:	ecaa                	sd	a0,88(sp)
    334e:	450d                	li	a0,3
    3350:	aadd                	j	3546 <.LBB232_62+0xc>
    3352:	02c6fd63          	bgeu	a3,a2,338c <.LBB232_56+0xe>
    3356:	0028                	addi	a0,sp,8
    3358:	e52a                	sd	a0,136(sp)

000000000000335a <.LBB232_54>:
    335a:	00001517          	auipc	a0,0x1
    335e:	95e50513          	addi	a0,a0,-1698 # 3cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>
    3362:	e92a                	sd	a0,144(sp)
    3364:	080c                	addi	a1,sp,16
    3366:	ed2e                	sd	a1,152(sp)
    3368:	f12a                	sd	a0,160(sp)
    336a:	0828                	addi	a0,sp,24
    336c:	f52a                	sd	a0,168(sp)

000000000000336e <.LBB232_55>:
    336e:	00001517          	auipc	a0,0x1
    3372:	b0e50513          	addi	a0,a0,-1266 # 3e7c <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hc6a9db97781cac35E>
    3376:	f92a                	sd	a0,176(sp)
    3378:	102c                	addi	a1,sp,40
    337a:	fd2e                	sd	a1,184(sp)
    337c:	e1aa                	sd	a0,192(sp)

000000000000337e <.LBB232_56>:
    337e:	00003517          	auipc	a0,0x3
    3382:	93250513          	addi	a0,a0,-1742 # 5cb0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.320>
    3386:	ecaa                	sd	a0,88(sp)
    3388:	4511                	li	a0,4
    338a:	aa75                	j	3546 <.LBB232_62+0xc>
    338c:	00163813          	seqz	a6,a2
    3390:	00c5c7b3          	xor	a5,a1,a2
    3394:	0017b793          	seqz	a5,a5
    3398:	00f867b3          	or	a5,a6,a5
    339c:	c399                	beqz	a5,33a2 <.LBB232_56+0x24>
    339e:	8636                	mv	a2,a3
    33a0:	a819                	j	33b6 <.LBB232_56+0x38>
    33a2:	00b67a63          	bgeu	a2,a1,33b6 <.LBB232_56+0x38>
    33a6:	00c507b3          	add	a5,a0,a2
    33aa:	00078803          	lb	a6,0(a5)
    33ae:	fc000793          	li	a5,-64
    33b2:	fef856e3          	bge	a6,a5,339e <.LBB232_56+0x20>
    33b6:	00163693          	seqz	a3,a2
    33ba:	00b647b3          	xor	a5,a2,a1
    33be:	0017b793          	seqz	a5,a5
    33c2:	8edd                	or	a3,a3,a5
    33c4:	fc32                	sd	a2,56(sp)
    33c6:	ca81                	beqz	a3,33d6 <.LBB232_56+0x58>
    33c8:	87b2                	mv	a5,a2
    33ca:	04410313          	addi	t1,sp,68
    33ce:	863e                	mv	a2,a5
    33d0:	02b60f63          	beq	a2,a1,340e <.LBB232_57>
    33d4:	a8a9                	j	342e <.LBB232_57+0x20>
    33d6:	00158893          	addi	a7,a1,1
    33da:	fc000813          	li	a6,-64
    33de:	00b67863          	bgeu	a2,a1,33ee <.LBB232_56+0x70>
    33e2:	00c507b3          	add	a5,a0,a2
    33e6:	00078783          	lb	a5,0(a5)
    33ea:	0307de63          	bge	a5,a6,3426 <.LBB232_57+0x18>
    33ee:	fff60793          	addi	a5,a2,-1
    33f2:	0017b693          	seqz	a3,a5
    33f6:	00c8c633          	xor	a2,a7,a2
    33fa:	00163613          	seqz	a2,a2
    33fe:	8ed1                	or	a3,a3,a2
    3400:	863e                	mv	a2,a5
    3402:	def1                	beqz	a3,33de <.LBB232_56+0x60>
    3404:	04410313          	addi	t1,sp,68
    3408:	863e                	mv	a2,a5
    340a:	02b61263          	bne	a2,a1,342e <.LBB232_57+0x20>

000000000000340e <.LBB232_57>:
    340e:	00002517          	auipc	a0,0x2
    3412:	3ea50513          	addi	a0,a0,1002 # 57f8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.169>
    3416:	02b00593          	li	a1,43
    341a:	863a                	mv	a2,a4
    341c:	fffff097          	auipc	ra,0xfffff
    3420:	b26080e7          	jalr	-1242(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
    3424:	0000                	unimp
    3426:	04410313          	addi	t1,sp,68
    342a:	feb602e3          	beq	a2,a1,340e <.LBB232_57>
    342e:	00c506b3          	add	a3,a0,a2
    3432:	00068783          	lb	a5,0(a3)
    3436:	0ff7f393          	andi	t2,a5,255
    343a:	0007c663          	bltz	a5,3446 <.LBB232_57+0x38>
    343e:	c29e                	sw	t2,68(sp)
    3440:	00a8                	addi	a0,sp,72
    3442:	4785                	li	a5,1
    3444:	a855                	j	34f8 <.LBB232_57+0xea>
    3446:	952e                	add	a0,a0,a1
    3448:	00168593          	addi	a1,a3,1
    344c:	00a59f63          	bne	a1,a0,346a <.LBB232_57+0x5c>
    3450:	4881                	li	a7,0
    3452:	85aa                	mv	a1,a0
    3454:	0e000693          	li	a3,224
    3458:	01f3f813          	andi	a6,t2,31
    345c:	02d3f363          	bgeu	t2,a3,3482 <.LBB232_57+0x74>
    3460:	00681513          	slli	a0,a6,0x6
    3464:	011565b3          	or	a1,a0,a7
    3468:	a0bd                	j	34d6 <.LBB232_57+0xc8>
    346a:	0016c783          	lbu	a5,1(a3)
    346e:	00268593          	addi	a1,a3,2
    3472:	03f7f893          	andi	a7,a5,63
    3476:	0e000693          	li	a3,224
    347a:	01f3f813          	andi	a6,t2,31
    347e:	fed3e1e3          	bltu	t2,a3,3460 <.LBB232_57+0x52>
    3482:	00a59563          	bne	a1,a0,348c <.LBB232_57+0x7e>
    3486:	4581                	li	a1,0
    3488:	82aa                	mv	t0,a0
    348a:	a039                	j	3498 <.LBB232_57+0x8a>
    348c:	0005c683          	lbu	a3,0(a1)
    3490:	00158293          	addi	t0,a1,1
    3494:	03f6f593          	andi	a1,a3,63
    3498:	00689693          	slli	a3,a7,0x6
    349c:	0f000793          	li	a5,240
    34a0:	8dd5                	or	a1,a1,a3
    34a2:	00f3e663          	bltu	t2,a5,34ae <.LBB232_57+0xa0>
    34a6:	00a29863          	bne	t0,a0,34b6 <.LBB232_57+0xa8>
    34aa:	4501                	li	a0,0
    34ac:	a809                	j	34be <.LBB232_57+0xb0>
    34ae:	00c81513          	slli	a0,a6,0xc
    34b2:	8dc9                	or	a1,a1,a0
    34b4:	a00d                	j	34d6 <.LBB232_57+0xc8>
    34b6:	0002c503          	lbu	a0,0(t0) # 110000 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x100e90>
    34ba:	03f57513          	andi	a0,a0,63
    34be:	01281693          	slli	a3,a6,0x12
    34c2:	001c07b7          	lui	a5,0x1c0
    34c6:	8efd                	and	a3,a3,a5
    34c8:	059a                	slli	a1,a1,0x6
    34ca:	8dd5                	or	a1,a1,a3
    34cc:	8dc9                	or	a1,a1,a0
    34ce:	00110537          	lui	a0,0x110
    34d2:	f2a58ee3          	beq	a1,a0,340e <.LBB232_57>
    34d6:	c2ae                	sw	a1,68(sp)
    34d8:	00a8                	addi	a0,sp,72
    34da:	08000693          	li	a3,128
    34de:	4785                	li	a5,1
    34e0:	00d5ec63          	bltu	a1,a3,34f8 <.LBB232_57+0xea>
    34e4:	00b5d693          	srli	a3,a1,0xb
    34e8:	4789                	li	a5,2
    34ea:	c699                	beqz	a3,34f8 <.LBB232_57+0xea>
    34ec:	81c1                	srli	a1,a1,0x10
    34ee:	0015b593          	seqz	a1,a1
    34f2:	4691                	li	a3,4
    34f4:	40b687b3          	sub	a5,a3,a1
    34f8:	00c785b3          	add	a1,a5,a2
    34fc:	e4b2                	sd	a2,72(sp)
    34fe:	e8ae                	sd	a1,80(sp)
    3500:	182c                	addi	a1,sp,56
    3502:	e52e                	sd	a1,136(sp)

0000000000003504 <.LBB232_58>:
    3504:	00000597          	auipc	a1,0x0
    3508:	7b458593          	addi	a1,a1,1972 # 3cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>
    350c:	e92e                	sd	a1,144(sp)
    350e:	ed1a                	sd	t1,152(sp)

0000000000003510 <.LBB232_59>:
    3510:	00000597          	auipc	a1,0x0
    3514:	93858593          	addi	a1,a1,-1736 # 2e48 <_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hef9725b75ac6bbf2E>
    3518:	f12e                	sd	a1,160(sp)
    351a:	f52a                	sd	a0,168(sp)

000000000000351c <.LBB232_60>:
    351c:	fffff517          	auipc	a0,0xfffff
    3520:	96c50513          	addi	a0,a0,-1684 # 1e88 <_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hf9fc33902e262dfeE>
    3524:	f92a                	sd	a0,176(sp)
    3526:	0828                	addi	a0,sp,24
    3528:	fd2a                	sd	a0,184(sp)

000000000000352a <.LBB232_61>:
    352a:	00001517          	auipc	a0,0x1
    352e:	95250513          	addi	a0,a0,-1710 # 3e7c <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hc6a9db97781cac35E>
    3532:	e1aa                	sd	a0,192(sp)
    3534:	102c                	addi	a1,sp,40
    3536:	e5ae                	sd	a1,200(sp)
    3538:	e9aa                	sd	a0,208(sp)

000000000000353a <.LBB232_62>:
    353a:	00002517          	auipc	a0,0x2
    353e:	7e650513          	addi	a0,a0,2022 # 5d20 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.324>
    3542:	ecaa                	sd	a0,88(sp)
    3544:	4515                	li	a0,5
    3546:	f0aa                	sd	a0,96(sp)
    3548:	f482                	sd	zero,104(sp)
    354a:	012c                	addi	a1,sp,136
    354c:	fcae                	sd	a1,120(sp)
    354e:	e12a                	sd	a0,128(sp)
    3550:	08a8                	addi	a0,sp,88
    3552:	85ba                	mv	a1,a4
    3554:	fffff097          	auipc	ra,0xfffff
    3558:	a5a080e7          	jalr	-1446(ra) # 1fae <_ZN4core9panicking9panic_fmt17ha96d96e8fa0883ecE>
	...

000000000000355e <_ZN4core7unicode9printable12is_printable17ha1b974ef4cc7a0dfE>:
    355e:	1141                	addi	sp,sp,-16
    3560:	e406                	sd	ra,8(sp)
    3562:	0105559b          	srliw	a1,a0,0x10
    3566:	e1f1                	bnez	a1,362a <.LBB248_43+0x4c>
    3568:	4581                	li	a1,0
    356a:	6641                	lui	a2,0x10
    356c:	f006061b          	addiw	a2,a2,-256
    3570:	8e69                	and	a2,a2,a0
    3572:	00865313          	srli	t1,a2,0x8

0000000000003576 <.LBB248_41>:
    3576:	00003717          	auipc	a4,0x3
    357a:	85270713          	addi	a4,a4,-1966 # 5dc8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.347>
    357e:	12300813          	li	a6,291

0000000000003582 <.LBB248_42>:
    3582:	00003897          	auipc	a7,0x3
    3586:	89888893          	addi	a7,a7,-1896 # 5e1a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348>
    358a:	05270293          	addi	t0,a4,82
    358e:	0ff57793          	andi	a5,a0,255
    3592:	a811                	j	35a6 <.LBB248_42+0x24>
    3594:	00d335b3          	sltu	a1,t1,a3
    3598:	00574633          	xor	a2,a4,t0
    359c:	00163613          	seqz	a2,a2
    35a0:	8e4d                	or	a2,a2,a1
    35a2:	859e                	mv	a1,t2
    35a4:	ea15                	bnez	a2,35d8 <.LBB248_42+0x56>
    35a6:	00074683          	lbu	a3,0(a4)
    35aa:	00174603          	lbu	a2,1(a4)
    35ae:	0709                	addi	a4,a4,2
    35b0:	00c583b3          	add	t2,a1,a2
    35b4:	fe6690e3          	bne	a3,t1,3594 <.LBB248_42+0x12>
    35b8:	1eb3e063          	bltu	t2,a1,3798 <.LBB248_47>
    35bc:	1f03f963          	bgeu	t2,a6,37ae <.LBB248_48>
    35c0:	95c6                	add	a1,a1,a7
    35c2:	ca01                	beqz	a2,35d2 <.LBB248_42+0x50>
    35c4:	0005c683          	lbu	a3,0(a1)
    35c8:	0585                	addi	a1,a1,1
    35ca:	167d                	addi	a2,a2,-1
    35cc:	fef69be3          	bne	a3,a5,35c2 <.LBB248_42+0x40>
    35d0:	a26d                	j	377a <.LBB248_46+0xd4>
    35d2:	859e                	mv	a1,t2
    35d4:	fc5719e3          	bne	a4,t0,35a6 <.LBB248_42+0x24>
    35d8:	65c1                	lui	a1,0x10
    35da:	35fd                	addiw	a1,a1,-1
    35dc:	8de9                	and	a1,a1,a0

00000000000035de <.LBB248_43>:
    35de:	00003717          	auipc	a4,0x3
    35e2:	95e70713          	addi	a4,a4,-1698 # 5f3c <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.349>
    35e6:	4505                	li	a0,1
    35e8:	587d                	li	a6,-1
    35ea:	13570693          	addi	a3,a4,309
    35ee:	00070783          	lb	a5,0(a4)
    35f2:	00170613          	addi	a2,a4,1
    35f6:	00f85963          	bge	a6,a5,3608 <.LBB248_43+0x2a>
    35fa:	0ff7f793          	andi	a5,a5,255
    35fe:	8732                	mv	a4,a2
    3600:	9d9d                	subw	a1,a1,a5
    3602:	0005df63          	bgez	a1,3620 <.LBB248_43+0x42>
    3606:	aa9d                	j	377c <.LBB248_46+0xd6>
    3608:	1cd60663          	beq	a2,a3,37d4 <.LBB248_50>
    360c:	00174603          	lbu	a2,1(a4)
    3610:	07f7f793          	andi	a5,a5,127
    3614:	07a2                	slli	a5,a5,0x8
    3616:	0709                	addi	a4,a4,2
    3618:	8fd1                	or	a5,a5,a2
    361a:	9d9d                	subw	a1,a1,a5
    361c:	1605c063          	bltz	a1,377c <.LBB248_46+0xd6>
    3620:	00154513          	xori	a0,a0,1
    3624:	fcd715e3          	bne	a4,a3,35ee <.LBB248_43+0x10>
    3628:	aa91                	j	377c <.LBB248_46+0xd6>
    362a:	0115559b          	srliw	a1,a0,0x11
    362e:	e1f1                	bnez	a1,36f2 <.LBB248_46+0x4c>
    3630:	4581                	li	a1,0
    3632:	6641                	lui	a2,0x10
    3634:	f006061b          	addiw	a2,a2,-256
    3638:	8e69                	and	a2,a2,a0
    363a:	00865313          	srli	t1,a2,0x8

000000000000363e <.LBB248_44>:
    363e:	00003717          	auipc	a4,0x3
    3642:	a3370713          	addi	a4,a4,-1485 # 6071 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.350>
    3646:	0b000813          	li	a6,176

000000000000364a <.LBB248_45>:
    364a:	00003897          	auipc	a7,0x3
    364e:	a7388893          	addi	a7,a7,-1421 # 60bd <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.351>
    3652:	04c70293          	addi	t0,a4,76
    3656:	0ff57793          	andi	a5,a0,255
    365a:	a811                	j	366e <.LBB248_45+0x24>
    365c:	00d335b3          	sltu	a1,t1,a3
    3660:	00574633          	xor	a2,a4,t0
    3664:	00163613          	seqz	a2,a2
    3668:	8e4d                	or	a2,a2,a1
    366a:	859e                	mv	a1,t2
    366c:	ea15                	bnez	a2,36a0 <.LBB248_45+0x56>
    366e:	00074683          	lbu	a3,0(a4)
    3672:	00174603          	lbu	a2,1(a4)
    3676:	0709                	addi	a4,a4,2
    3678:	00c583b3          	add	t2,a1,a2
    367c:	fe6690e3          	bne	a3,t1,365c <.LBB248_45+0x12>
    3680:	10b3ec63          	bltu	t2,a1,3798 <.LBB248_47>
    3684:	1303fc63          	bgeu	t2,a6,37bc <.LBB248_49>
    3688:	95c6                	add	a1,a1,a7
    368a:	ca01                	beqz	a2,369a <.LBB248_45+0x50>
    368c:	0005c683          	lbu	a3,0(a1) # 10000 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xe90>
    3690:	0585                	addi	a1,a1,1
    3692:	167d                	addi	a2,a2,-1
    3694:	fef69be3          	bne	a3,a5,368a <.LBB248_45+0x40>
    3698:	a0cd                	j	377a <.LBB248_46+0xd4>
    369a:	859e                	mv	a1,t2
    369c:	fc5719e3          	bne	a4,t0,366e <.LBB248_45+0x24>
    36a0:	65c1                	lui	a1,0x10
    36a2:	35fd                	addiw	a1,a1,-1
    36a4:	8de9                	and	a1,a1,a0

00000000000036a6 <.LBB248_46>:
    36a6:	00003717          	auipc	a4,0x3
    36aa:	ac670713          	addi	a4,a4,-1338 # 616c <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.352>
    36ae:	4505                	li	a0,1
    36b0:	587d                	li	a6,-1
    36b2:	1a370693          	addi	a3,a4,419
    36b6:	00070783          	lb	a5,0(a4)
    36ba:	00170613          	addi	a2,a4,1
    36be:	00f85963          	bge	a6,a5,36d0 <.LBB248_46+0x2a>
    36c2:	0ff7f793          	andi	a5,a5,255
    36c6:	8732                	mv	a4,a2
    36c8:	9d9d                	subw	a1,a1,a5
    36ca:	0005df63          	bgez	a1,36e8 <.LBB248_46+0x42>
    36ce:	a07d                	j	377c <.LBB248_46+0xd6>
    36d0:	10d60263          	beq	a2,a3,37d4 <.LBB248_50>
    36d4:	00174603          	lbu	a2,1(a4)
    36d8:	07f7f793          	andi	a5,a5,127
    36dc:	07a2                	slli	a5,a5,0x8
    36de:	0709                	addi	a4,a4,2
    36e0:	8fd1                	or	a5,a5,a2
    36e2:	9d9d                	subw	a1,a1,a5
    36e4:	0805cc63          	bltz	a1,377c <.LBB248_46+0xd6>
    36e8:	00154513          	xori	a0,a0,1
    36ec:	fcd715e3          	bne	a4,a3,36b6 <.LBB248_46+0x10>
    36f0:	a071                	j	377c <.LBB248_46+0xd6>
    36f2:	fffd65b7          	lui	a1,0xfffd6
    36f6:	9225859b          	addiw	a1,a1,-1758
    36fa:	9da9                	addw	a1,a1,a0
    36fc:	0225b593          	sltiu	a1,a1,34
    3700:	fffd5637          	lui	a2,0xfffd5
    3704:	8cb6061b          	addiw	a2,a2,-1845
    3708:	9e29                	addw	a2,a2,a0
    370a:	00b63613          	sltiu	a2,a2,11
    370e:	8dd1                	or	a1,a1,a2
    3710:	00200637          	lui	a2,0x200
    3714:	3679                	addiw	a2,a2,-2
    3716:	8e69                	and	a2,a2,a0
    3718:	0002c6b7          	lui	a3,0x2c
    371c:	81e6869b          	addiw	a3,a3,-2018
    3720:	8e35                	xor	a2,a2,a3
    3722:	00163613          	seqz	a2,a2
    3726:	8dd1                	or	a1,a1,a2
    3728:	fffd3637          	lui	a2,0xfffd3
    372c:	15e6061b          	addiw	a2,a2,350
    3730:	9e29                	addw	a2,a2,a0
    3732:	00e63613          	sltiu	a2,a2,14
    3736:	8dd1                	or	a1,a1,a2
    3738:	fffd1637          	lui	a2,0xfffd1
    373c:	41f6061b          	addiw	a2,a2,1055
    3740:	9e29                	addw	a2,a2,a0
    3742:	6685                	lui	a3,0x1
    3744:	c1f6869b          	addiw	a3,a3,-993
    3748:	00d63633          	sltu	a2,a2,a3
    374c:	8dd1                	or	a1,a1,a2
    374e:	fffd0637          	lui	a2,0xfffd0
    3752:	5e26061b          	addiw	a2,a2,1506
    3756:	9e29                	addw	a2,a2,a0
    3758:	5e263613          	sltiu	a2,a2,1506
    375c:	8dd1                	or	a1,a1,a2
    375e:	fffcf637          	lui	a2,0xfffcf
    3762:	cb56061b          	addiw	a2,a2,-843
    3766:	9e29                	addw	a2,a2,a0
    3768:	000af6b7          	lui	a3,0xaf
    376c:	db56869b          	addiw	a3,a3,-587
    3770:	00d63633          	sltu	a2,a2,a3
    3774:	8dd1                	or	a1,a1,a2
    3776:	8985                	andi	a1,a1,1
    3778:	c591                	beqz	a1,3784 <.LBB248_46+0xde>
    377a:	4501                	li	a0,0
    377c:	8905                	andi	a0,a0,1
    377e:	60a2                	ld	ra,8(sp)
    3780:	0141                	addi	sp,sp,16
    3782:	8082                	ret
    3784:	2501                	sext.w	a0,a0
    3786:	000e05b7          	lui	a1,0xe0
    378a:	1f05859b          	addiw	a1,a1,496
    378e:	00b53533          	sltu	a0,a0,a1
    3792:	60a2                	ld	ra,8(sp)
    3794:	0141                	addi	sp,sp,16
    3796:	8082                	ret

0000000000003798 <.LBB248_47>:
    3798:	00002617          	auipc	a2,0x2
    379c:	60060613          	addi	a2,a2,1536 # 5d98 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.345>
    37a0:	852e                	mv	a0,a1
    37a2:	859e                	mv	a1,t2
    37a4:	00000097          	auipc	ra,0x0
    37a8:	aa0080e7          	jalr	-1376(ra) # 3244 <_ZN4core5slice5index22slice_index_order_fail17h2f93cb17ebf66956E>
	...

00000000000037ae <.LBB248_48>:
    37ae:	00002617          	auipc	a2,0x2
    37b2:	5ea60613          	addi	a2,a2,1514 # 5d98 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.345>
    37b6:	12200593          	li	a1,290
    37ba:	a039                	j	37c8 <.LBB248_49+0xc>

00000000000037bc <.LBB248_49>:
    37bc:	00002617          	auipc	a2,0x2
    37c0:	5dc60613          	addi	a2,a2,1500 # 5d98 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.345>
    37c4:	0af00593          	li	a1,175
    37c8:	851e                	mv	a0,t2
    37ca:	00000097          	auipc	ra,0x0
    37ce:	a3a080e7          	jalr	-1478(ra) # 3204 <_ZN4core5slice5index24slice_end_index_len_fail17h0db61fbd8d9e0e45E>
	...

00000000000037d4 <.LBB248_50>:
    37d4:	00002517          	auipc	a0,0x2
    37d8:	02450513          	addi	a0,a0,36 # 57f8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.169>

00000000000037dc <.LBB248_51>:
    37dc:	00002617          	auipc	a2,0x2
    37e0:	5d460613          	addi	a2,a2,1492 # 5db0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.346>
    37e4:	02b00593          	li	a1,43
    37e8:	ffffe097          	auipc	ra,0xffffe
    37ec:	75a080e7          	jalr	1882(ra) # 1f42 <_ZN4core9panicking5panic17h2fce00465d999e0cE>
	...

00000000000037f2 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h2b088f06fe0c0eb7E>:
    37f2:	7175                	addi	sp,sp,-144
    37f4:	e506                	sd	ra,136(sp)
    37f6:	882e                	mv	a6,a1
    37f8:	4581                	li	a1,0
    37fa:	00056703          	lwu	a4,0(a0)
    37fe:	00810893          	addi	a7,sp,8
    3802:	10000537          	lui	a0,0x10000
    3806:	357d                	addiw	a0,a0,-1
    3808:	42a9                	li	t0,10
    380a:	a039                	j	3818 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h2b088f06fe0c0eb7E+0x26>
    380c:	05760613          	addi	a2,a2,87
    3810:	06c78fa3          	sb	a2,127(a5) # 1c007f <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1b0f0f>
    3814:	15fd                	addi	a1,a1,-1
    3816:	c30d                	beqz	a4,3838 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h2b088f06fe0c0eb7E+0x46>
    3818:	00b887b3          	add	a5,a7,a1
    381c:	00475693          	srli	a3,a4,0x4
    3820:	00f77613          	andi	a2,a4,15
    3824:	00a6f733          	and	a4,a3,a0
    3828:	fe5672e3          	bgeu	a2,t0,380c <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h2b088f06fe0c0eb7E+0x1a>
    382c:	03066613          	ori	a2,a2,48
    3830:	06c78fa3          	sb	a2,127(a5)
    3834:	15fd                	addi	a1,a1,-1
    3836:	f36d                	bnez	a4,3818 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h2b088f06fe0c0eb7E+0x26>
    3838:	08058513          	addi	a0,a1,128 # e0080 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xd0f10>
    383c:	08100613          	li	a2,129
    3840:	02c57663          	bgeu	a0,a2,386c <.LBB457_8>
    3844:	40b007b3          	neg	a5,a1
    3848:	00b88533          	add	a0,a7,a1
    384c:	08050713          	addi	a4,a0,128 # 10000080 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xfff0f10>

0000000000003850 <.LBB457_7>:
    3850:	00002617          	auipc	a2,0x2
    3854:	21860613          	addi	a2,a2,536 # 5a68 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.226>
    3858:	4585                	li	a1,1
    385a:	4689                	li	a3,2
    385c:	8542                	mv	a0,a6
    385e:	fffff097          	auipc	ra,0xfffff
    3862:	026080e7          	jalr	38(ra) # 2884 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E>
    3866:	60aa                	ld	ra,136(sp)
    3868:	6149                	addi	sp,sp,144
    386a:	8082                	ret

000000000000386c <.LBB457_8>:
    386c:	00002617          	auipc	a2,0x2
    3870:	1e460613          	addi	a2,a2,484 # 5a50 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.224>
    3874:	08000593          	li	a1,128
    3878:	00000097          	auipc	ra,0x0
    387c:	94c080e7          	jalr	-1716(ra) # 31c4 <_ZN4core5slice5index26slice_start_index_len_fail17hf5a8e79169c741aaE>
	...

0000000000003882 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hc5ad538a9a7d4dd4E>:
    3882:	7175                	addi	sp,sp,-144
    3884:	e506                	sd	ra,136(sp)
    3886:	882e                	mv	a6,a1
    3888:	4581                	li	a1,0
    388a:	00056703          	lwu	a4,0(a0)
    388e:	00810893          	addi	a7,sp,8
    3892:	10000537          	lui	a0,0x10000
    3896:	357d                	addiw	a0,a0,-1
    3898:	42a9                	li	t0,10
    389a:	a039                	j	38a8 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hc5ad538a9a7d4dd4E+0x26>
    389c:	03760613          	addi	a2,a2,55
    38a0:	06c78fa3          	sb	a2,127(a5)
    38a4:	15fd                	addi	a1,a1,-1
    38a6:	c30d                	beqz	a4,38c8 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hc5ad538a9a7d4dd4E+0x46>
    38a8:	00b887b3          	add	a5,a7,a1
    38ac:	00475693          	srli	a3,a4,0x4
    38b0:	00f77613          	andi	a2,a4,15
    38b4:	00a6f733          	and	a4,a3,a0
    38b8:	fe5672e3          	bgeu	a2,t0,389c <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hc5ad538a9a7d4dd4E+0x1a>
    38bc:	03066613          	ori	a2,a2,48
    38c0:	06c78fa3          	sb	a2,127(a5)
    38c4:	15fd                	addi	a1,a1,-1
    38c6:	f36d                	bnez	a4,38a8 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hc5ad538a9a7d4dd4E+0x26>
    38c8:	08058513          	addi	a0,a1,128
    38cc:	08100613          	li	a2,129
    38d0:	02c57663          	bgeu	a0,a2,38fc <.LBB458_8>
    38d4:	40b007b3          	neg	a5,a1
    38d8:	00b88533          	add	a0,a7,a1
    38dc:	08050713          	addi	a4,a0,128 # 10000080 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xfff0f10>

00000000000038e0 <.LBB458_7>:
    38e0:	00002617          	auipc	a2,0x2
    38e4:	18860613          	addi	a2,a2,392 # 5a68 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.226>
    38e8:	4585                	li	a1,1
    38ea:	4689                	li	a3,2
    38ec:	8542                	mv	a0,a6
    38ee:	fffff097          	auipc	ra,0xfffff
    38f2:	f96080e7          	jalr	-106(ra) # 2884 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E>
    38f6:	60aa                	ld	ra,136(sp)
    38f8:	6149                	addi	sp,sp,144
    38fa:	8082                	ret

00000000000038fc <.LBB458_8>:
    38fc:	00002617          	auipc	a2,0x2
    3900:	15460613          	addi	a2,a2,340 # 5a50 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.224>
    3904:	08000593          	li	a1,128
    3908:	00000097          	auipc	ra,0x0
    390c:	8bc080e7          	jalr	-1860(ra) # 31c4 <_ZN4core5slice5index26slice_start_index_len_fail17hf5a8e79169c741aaE>
	...

0000000000003912 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h68aa87d5d39609c1E>:
    3912:	7175                	addi	sp,sp,-144
    3914:	e506                	sd	ra,136(sp)
    3916:	6114                	ld	a3,0(a0)
    3918:	852e                	mv	a0,a1
    391a:	4581                	li	a1,0
    391c:	00810813          	addi	a6,sp,8
    3920:	4729                	li	a4,10
    3922:	a039                	j	3930 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h68aa87d5d39609c1E+0x1e>
    3924:	05760613          	addi	a2,a2,87
    3928:	06c78fa3          	sb	a2,127(a5)
    392c:	15fd                	addi	a1,a1,-1
    392e:	ce91                	beqz	a3,394a <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h68aa87d5d39609c1E+0x38>
    3930:	00b807b3          	add	a5,a6,a1
    3934:	00f6f613          	andi	a2,a3,15
    3938:	8291                	srli	a3,a3,0x4
    393a:	fee675e3          	bgeu	a2,a4,3924 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h68aa87d5d39609c1E+0x12>
    393e:	03066613          	ori	a2,a2,48
    3942:	06c78fa3          	sb	a2,127(a5)
    3946:	15fd                	addi	a1,a1,-1
    3948:	f6e5                	bnez	a3,3930 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h68aa87d5d39609c1E+0x1e>
    394a:	08058693          	addi	a3,a1,128
    394e:	08100613          	li	a2,129
    3952:	02c6f463          	bgeu	a3,a2,397a <.LBB461_8>
    3956:	40b007b3          	neg	a5,a1
    395a:	95c2                	add	a1,a1,a6
    395c:	08058713          	addi	a4,a1,128

0000000000003960 <.LBB461_7>:
    3960:	00002617          	auipc	a2,0x2
    3964:	10860613          	addi	a2,a2,264 # 5a68 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.226>
    3968:	4585                	li	a1,1
    396a:	4689                	li	a3,2
    396c:	fffff097          	auipc	ra,0xfffff
    3970:	f18080e7          	jalr	-232(ra) # 2884 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E>
    3974:	60aa                	ld	ra,136(sp)
    3976:	6149                	addi	sp,sp,144
    3978:	8082                	ret

000000000000397a <.LBB461_8>:
    397a:	00002617          	auipc	a2,0x2
    397e:	0d660613          	addi	a2,a2,214 # 5a50 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.224>
    3982:	08000593          	li	a1,128
    3986:	8536                	mv	a0,a3
    3988:	00000097          	auipc	ra,0x0
    398c:	83c080e7          	jalr	-1988(ra) # 31c4 <_ZN4core5slice5index26slice_start_index_len_fail17hf5a8e79169c741aaE>
	...

0000000000003992 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h898848ae100a065bE>:
    3992:	7175                	addi	sp,sp,-144
    3994:	e506                	sd	ra,136(sp)
    3996:	6114                	ld	a3,0(a0)
    3998:	852e                	mv	a0,a1
    399a:	4581                	li	a1,0
    399c:	00810813          	addi	a6,sp,8
    39a0:	4729                	li	a4,10
    39a2:	a039                	j	39b0 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h898848ae100a065bE+0x1e>
    39a4:	03760613          	addi	a2,a2,55
    39a8:	06c78fa3          	sb	a2,127(a5)
    39ac:	15fd                	addi	a1,a1,-1
    39ae:	ce91                	beqz	a3,39ca <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h898848ae100a065bE+0x38>
    39b0:	00b807b3          	add	a5,a6,a1
    39b4:	00f6f613          	andi	a2,a3,15
    39b8:	8291                	srli	a3,a3,0x4
    39ba:	fee675e3          	bgeu	a2,a4,39a4 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h898848ae100a065bE+0x12>
    39be:	03066613          	ori	a2,a2,48
    39c2:	06c78fa3          	sb	a2,127(a5)
    39c6:	15fd                	addi	a1,a1,-1
    39c8:	f6e5                	bnez	a3,39b0 <_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h898848ae100a065bE+0x1e>
    39ca:	08058693          	addi	a3,a1,128
    39ce:	08100613          	li	a2,129
    39d2:	02c6f463          	bgeu	a3,a2,39fa <.LBB462_8>
    39d6:	40b007b3          	neg	a5,a1
    39da:	95c2                	add	a1,a1,a6
    39dc:	08058713          	addi	a4,a1,128

00000000000039e0 <.LBB462_7>:
    39e0:	00002617          	auipc	a2,0x2
    39e4:	08860613          	addi	a2,a2,136 # 5a68 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.226>
    39e8:	4585                	li	a1,1
    39ea:	4689                	li	a3,2
    39ec:	fffff097          	auipc	ra,0xfffff
    39f0:	e98080e7          	jalr	-360(ra) # 2884 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E>
    39f4:	60aa                	ld	ra,136(sp)
    39f6:	6149                	addi	sp,sp,144
    39f8:	8082                	ret

00000000000039fa <.LBB462_8>:
    39fa:	00002617          	auipc	a2,0x2
    39fe:	05660613          	addi	a2,a2,86 # 5a50 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.224>
    3a02:	08000593          	li	a1,128
    3a06:	8536                	mv	a0,a3
    3a08:	fffff097          	auipc	ra,0xfffff
    3a0c:	7bc080e7          	jalr	1980(ra) # 31c4 <_ZN4core5slice5index26slice_start_index_len_fail17hf5a8e79169c741aaE>
	...

0000000000003a12 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E>:
    3a12:	7175                	addi	sp,sp,-144
    3a14:	e506                	sd	ra,136(sp)
    3a16:	882e                	mv	a6,a1
    3a18:	0305e583          	lwu	a1,48(a1)
    3a1c:	0105f613          	andi	a2,a1,16
    3a20:	ea19                	bnez	a2,3a36 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x24>
    3a22:	0205f593          	andi	a1,a1,32
    3a26:	e1a9                	bnez	a1,3a68 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x56>
    3a28:	85c2                	mv	a1,a6
    3a2a:	60aa                	ld	ra,136(sp)
    3a2c:	6149                	addi	sp,sp,144
    3a2e:	00000317          	auipc	t1,0x0
    3a32:	28a30067          	jr	650(t1) # 3cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>
    3a36:	6108                	ld	a0,0(a0)
    3a38:	4581                	li	a1,0
    3a3a:	0030                	addi	a2,sp,8
    3a3c:	46a9                	li	a3,10
    3a3e:	a039                	j	3a4c <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x3a>
    3a40:	05778793          	addi	a5,a5,87
    3a44:	06f70fa3          	sb	a5,127(a4)
    3a48:	15fd                	addi	a1,a1,-1
    3a4a:	c539                	beqz	a0,3a98 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x86>
    3a4c:	00b60733          	add	a4,a2,a1
    3a50:	00f57793          	andi	a5,a0,15
    3a54:	8111                	srli	a0,a0,0x4
    3a56:	fed7f5e3          	bgeu	a5,a3,3a40 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x2e>
    3a5a:	0307e793          	ori	a5,a5,48
    3a5e:	06f70fa3          	sb	a5,127(a4)
    3a62:	15fd                	addi	a1,a1,-1
    3a64:	f565                	bnez	a0,3a4c <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x3a>
    3a66:	a80d                	j	3a98 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x86>
    3a68:	6108                	ld	a0,0(a0)
    3a6a:	4581                	li	a1,0
    3a6c:	0030                	addi	a2,sp,8
    3a6e:	46a9                	li	a3,10
    3a70:	a039                	j	3a7e <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x6c>
    3a72:	03778793          	addi	a5,a5,55
    3a76:	06f70fa3          	sb	a5,127(a4)
    3a7a:	15fd                	addi	a1,a1,-1
    3a7c:	cd11                	beqz	a0,3a98 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x86>
    3a7e:	00b60733          	add	a4,a2,a1
    3a82:	00f57793          	andi	a5,a0,15
    3a86:	8111                	srli	a0,a0,0x4
    3a88:	fed7f5e3          	bgeu	a5,a3,3a72 <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x60>
    3a8c:	0307e793          	ori	a5,a5,48
    3a90:	06f70fa3          	sb	a5,127(a4)
    3a94:	15fd                	addi	a1,a1,-1
    3a96:	f565                	bnez	a0,3a7e <_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h34cb342226214140E+0x6c>
    3a98:	08058513          	addi	a0,a1,128
    3a9c:	08100693          	li	a3,129
    3aa0:	02d57663          	bgeu	a0,a3,3acc <.LBB467_15>
    3aa4:	40b007b3          	neg	a5,a1
    3aa8:	00b60533          	add	a0,a2,a1
    3aac:	08050713          	addi	a4,a0,128

0000000000003ab0 <.LBB467_14>:
    3ab0:	00002617          	auipc	a2,0x2
    3ab4:	fb860613          	addi	a2,a2,-72 # 5a68 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.226>
    3ab8:	4585                	li	a1,1
    3aba:	4689                	li	a3,2
    3abc:	8542                	mv	a0,a6
    3abe:	fffff097          	auipc	ra,0xfffff
    3ac2:	dc6080e7          	jalr	-570(ra) # 2884 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E>
    3ac6:	60aa                	ld	ra,136(sp)
    3ac8:	6149                	addi	sp,sp,144
    3aca:	8082                	ret

0000000000003acc <.LBB467_15>:
    3acc:	00002617          	auipc	a2,0x2
    3ad0:	f8460613          	addi	a2,a2,-124 # 5a50 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.224>
    3ad4:	08000593          	li	a1,128
    3ad8:	fffff097          	auipc	ra,0xfffff
    3adc:	6ec080e7          	jalr	1772(ra) # 31c4 <_ZN4core5slice5index26slice_start_index_len_fail17hf5a8e79169c741aaE>
	...

0000000000003ae2 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h4cbd83af9447a2a8E>:
    3ae2:	715d                	addi	sp,sp,-80
    3ae4:	e486                	sd	ra,72(sp)
    3ae6:	e0a2                	sd	s0,64(sp)
    3ae8:	fc26                	sd	s1,56(sp)
    3aea:	f84a                	sd	s2,48(sp)
    3aec:	f44e                	sd	s3,40(sp)
    3aee:	00052803          	lw	a6,0(a0)
    3af2:	852e                	mv	a0,a1
    3af4:	43f85593          	srai	a1,a6,0x3f
    3af8:	00b80633          	add	a2,a6,a1
    3afc:	00b649b3          	xor	s3,a2,a1
    3b00:	0049d593          	srli	a1,s3,0x4
    3b04:	02700693          	li	a3,39
    3b08:	27100613          	li	a2,625

0000000000003b0c <.LBB472_10>:
    3b0c:	00002897          	auipc	a7,0x2
    3b10:	f5e88893          	addi	a7,a7,-162 # 5a6a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.228>
    3b14:	02c5f663          	bgeu	a1,a2,3b40 <.LBB472_10+0x34>
    3b18:	06300613          	li	a2,99
    3b1c:	55fd                	li	a1,-1
    3b1e:	0f366263          	bltu	a2,s3,3c02 <.LBB472_10+0xf6>
    3b22:	4625                	li	a2,9
    3b24:	0105a5b3          	slt	a1,a1,a6
    3b28:	13364f63          	blt	a2,s3,3c66 <.LBB472_10+0x15a>
    3b2c:	fff68613          	addi	a2,a3,-1 # aefff <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x9fe8f>
    3b30:	00110693          	addi	a3,sp,1
    3b34:	96b2                	add	a3,a3,a2
    3b36:	03098713          	addi	a4,s3,48
    3b3a:	00e68023          	sb	a4,0(a3)
    3b3e:	a2a9                	j	3c88 <.LBB472_10+0x17c>
    3b40:	4581                	li	a1,0
    3b42:	001a3637          	lui	a2,0x1a3
    3b46:	6e36061b          	addiw	a2,a2,1763
    3b4a:	063a                	slli	a2,a2,0xe
    3b4c:	ac760613          	addi	a2,a2,-1337 # 1a2ac7 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x193957>
    3b50:	063a                	slli	a2,a2,0xe
    3b52:	43360613          	addi	a2,a2,1075
    3b56:	0636                	slli	a2,a2,0xd
    3b58:	94b60293          	addi	t0,a2,-1717
    3b5c:	6609                	lui	a2,0x2
    3b5e:	7106039b          	addiw	t2,a2,1808
    3b62:	6641                	lui	a2,0x10
    3b64:	ffc60e9b          	addiw	t4,a2,-4
    3b68:	0051f6b7          	lui	a3,0x51f
    3b6c:	b856869b          	addiw	a3,a3,-1147
    3b70:	06b6                	slli	a3,a3,0xd
    3b72:	3d768693          	addi	a3,a3,983 # 51f3d7 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x510267>
    3b76:	06ba                	slli	a3,a3,0xe
    3b78:	28f68693          	addi	a3,a3,655
    3b7c:	06b2                	slli	a3,a3,0xc
    3b7e:	5c368f13          	addi	t5,a3,1475
    3b82:	06400313          	li	t1,100
    3b86:	ffe60f9b          	addiw	t6,a2,-2
    3b8a:	00110e13          	addi	t3,sp,1
    3b8e:	05f5e6b7          	lui	a3,0x5f5e
    3b92:	0ff6891b          	addiw	s2,a3,255
    3b96:	874e                	mv	a4,s3
    3b98:	0259b7b3          	mulhu	a5,s3,t0
    3b9c:	00b7d993          	srli	s3,a5,0xb
    3ba0:	02798633          	mul	a2,s3,t2
    3ba4:	40c70633          	sub	a2,a4,a2
    3ba8:	01d67433          	and	s0,a2,t4
    3bac:	8009                	srli	s0,s0,0x2
    3bae:	03e43433          	mulhu	s0,s0,t5
    3bb2:	8009                	srli	s0,s0,0x2
    3bb4:	00141493          	slli	s1,s0,0x1
    3bb8:	02640433          	mul	s0,s0,t1
    3bbc:	8e01                	sub	a2,a2,s0
    3bbe:	0606                	slli	a2,a2,0x1
    3bc0:	01f67633          	and	a2,a2,t6
    3bc4:	01148433          	add	s0,s1,a7
    3bc8:	00be04b3          	add	s1,t3,a1
    3bcc:	00044683          	lbu	a3,0(s0)
    3bd0:	00140403          	lb	s0,1(s0)
    3bd4:	9646                	add	a2,a2,a7
    3bd6:	00160783          	lb	a5,1(a2) # 10001 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xe91>
    3bda:	00064603          	lbu	a2,0(a2)
    3bde:	02848223          	sb	s0,36(s1)
    3be2:	02d481a3          	sb	a3,35(s1)
    3be6:	02f48323          	sb	a5,38(s1)
    3bea:	02c482a3          	sb	a2,37(s1)
    3bee:	15f1                	addi	a1,a1,-4
    3bf0:	fae963e3          	bltu	s2,a4,3b96 <.LBB472_10+0x8a>
    3bf4:	02758693          	addi	a3,a1,39
    3bf8:	06300613          	li	a2,99
    3bfc:	55fd                	li	a1,-1
    3bfe:	f33672e3          	bgeu	a2,s3,3b22 <.LBB472_10+0x16>
    3c02:	6641                	lui	a2,0x10
    3c04:	ffc6071b          	addiw	a4,a2,-4
    3c08:	00e9f733          	and	a4,s3,a4
    3c0c:	8309                	srli	a4,a4,0x2
    3c0e:	0051f7b7          	lui	a5,0x51f
    3c12:	b857879b          	addiw	a5,a5,-1147
    3c16:	07b6                	slli	a5,a5,0xd
    3c18:	3d778793          	addi	a5,a5,983 # 51f3d7 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x510267>
    3c1c:	07ba                	slli	a5,a5,0xe
    3c1e:	28f78793          	addi	a5,a5,655
    3c22:	07b2                	slli	a5,a5,0xc
    3c24:	5c378793          	addi	a5,a5,1475
    3c28:	02f73733          	mulhu	a4,a4,a5
    3c2c:	8309                	srli	a4,a4,0x2
    3c2e:	06400793          	li	a5,100
    3c32:	02f707b3          	mul	a5,a4,a5
    3c36:	40f987b3          	sub	a5,s3,a5
    3c3a:	0786                	slli	a5,a5,0x1
    3c3c:	3679                	addiw	a2,a2,-2
    3c3e:	8e7d                	and	a2,a2,a5
    3c40:	16f9                	addi	a3,a3,-2
    3c42:	9646                	add	a2,a2,a7
    3c44:	00160783          	lb	a5,1(a2) # 10001 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xe91>
    3c48:	00064603          	lbu	a2,0(a2)
    3c4c:	00110493          	addi	s1,sp,1
    3c50:	94b6                	add	s1,s1,a3
    3c52:	00f480a3          	sb	a5,1(s1)
    3c56:	00c48023          	sb	a2,0(s1)
    3c5a:	89ba                	mv	s3,a4
    3c5c:	4625                	li	a2,9
    3c5e:	0105a5b3          	slt	a1,a1,a6
    3c62:	ed3655e3          	bge	a2,s3,3b2c <.LBB472_10+0x20>
    3c66:	00199713          	slli	a4,s3,0x1
    3c6a:	ffe68613          	addi	a2,a3,-2 # 5f5dffe <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x5f4ee8e>
    3c6e:	011706b3          	add	a3,a4,a7
    3c72:	00168703          	lb	a4,1(a3)
    3c76:	0006c683          	lbu	a3,0(a3)
    3c7a:	00110793          	addi	a5,sp,1
    3c7e:	97b2                	add	a5,a5,a2
    3c80:	00e780a3          	sb	a4,1(a5)
    3c84:	00d78023          	sb	a3,0(a5)
    3c88:	00110693          	addi	a3,sp,1
    3c8c:	00c68733          	add	a4,a3,a2
    3c90:	02700693          	li	a3,39
    3c94:	40c687b3          	sub	a5,a3,a2

0000000000003c98 <.LBB472_11>:
    3c98:	00002617          	auipc	a2,0x2
    3c9c:	b4860613          	addi	a2,a2,-1208 # 57e0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.134>
    3ca0:	4681                	li	a3,0
    3ca2:	fffff097          	auipc	ra,0xfffff
    3ca6:	be2080e7          	jalr	-1054(ra) # 2884 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E>
    3caa:	79a2                	ld	s3,40(sp)
    3cac:	7942                	ld	s2,48(sp)
    3cae:	74e2                	ld	s1,56(sp)
    3cb0:	6406                	ld	s0,64(sp)
    3cb2:	60a6                	ld	ra,72(sp)
    3cb4:	6161                	addi	sp,sp,80
    3cb6:	8082                	ret

0000000000003cb8 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17hb54b9f30c2999dbcE>:
    3cb8:	715d                	addi	sp,sp,-80
    3cba:	e486                	sd	ra,72(sp)
    3cbc:	e0a2                	sd	s0,64(sp)
    3cbe:	fc26                	sd	s1,56(sp)
    3cc0:	f84a                	sd	s2,48(sp)
    3cc2:	00053903          	ld	s2,0(a0)
    3cc6:	852e                	mv	a0,a1
    3cc8:	00495593          	srli	a1,s2,0x4
    3ccc:	02700693          	li	a3,39
    3cd0:	27100713          	li	a4,625

0000000000003cd4 <.LBB475_10>:
    3cd4:	00002817          	auipc	a6,0x2
    3cd8:	d9680813          	addi	a6,a6,-618 # 5a6a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.228>
    3cdc:	02e5f363          	bgeu	a1,a4,3d02 <.LBB475_10+0x2e>
    3ce0:	06300593          	li	a1,99
    3ce4:	0f25c063          	blt	a1,s2,3dc4 <.LBB475_10+0xf0>
    3ce8:	45a5                	li	a1,9
    3cea:	1325cd63          	blt	a1,s2,3e24 <.LBB475_10+0x150>
    3cee:	fff68593          	addi	a1,a3,-1
    3cf2:	00910613          	addi	a2,sp,9
    3cf6:	962e                	add	a2,a2,a1
    3cf8:	03090693          	addi	a3,s2,48
    3cfc:	00d60023          	sb	a3,0(a2)
    3d00:	a291                	j	3e44 <.LBB475_10+0x170>
    3d02:	4681                	li	a3,0
    3d04:	001a35b7          	lui	a1,0x1a3
    3d08:	6e35859b          	addiw	a1,a1,1763
    3d0c:	05ba                	slli	a1,a1,0xe
    3d0e:	ac758593          	addi	a1,a1,-1337 # 1a2ac7 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x193957>
    3d12:	05ba                	slli	a1,a1,0xe
    3d14:	43358593          	addi	a1,a1,1075
    3d18:	05b6                	slli	a1,a1,0xd
    3d1a:	94b58893          	addi	a7,a1,-1717
    3d1e:	6589                	lui	a1,0x2
    3d20:	7105831b          	addiw	t1,a1,1808
    3d24:	65c1                	lui	a1,0x10
    3d26:	ffc58e1b          	addiw	t3,a1,-4
    3d2a:	0051f737          	lui	a4,0x51f
    3d2e:	b857071b          	addiw	a4,a4,-1147
    3d32:	0736                	slli	a4,a4,0xd
    3d34:	3d770713          	addi	a4,a4,983 # 51f3d7 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x510267>
    3d38:	073a                	slli	a4,a4,0xe
    3d3a:	28f70713          	addi	a4,a4,655
    3d3e:	0732                	slli	a4,a4,0xc
    3d40:	5c370e93          	addi	t4,a4,1475
    3d44:	06400293          	li	t0,100
    3d48:	ffe58f1b          	addiw	t5,a1,-2
    3d4c:	00910393          	addi	t2,sp,9
    3d50:	05f5e5b7          	lui	a1,0x5f5e
    3d54:	0ff58f9b          	addiw	t6,a1,255
    3d58:	864a                	mv	a2,s2
    3d5a:	031937b3          	mulhu	a5,s2,a7
    3d5e:	00b7d913          	srli	s2,a5,0xb
    3d62:	02690733          	mul	a4,s2,t1
    3d66:	40e60733          	sub	a4,a2,a4
    3d6a:	01c775b3          	and	a1,a4,t3
    3d6e:	8189                	srli	a1,a1,0x2
    3d70:	03d5b5b3          	mulhu	a1,a1,t4
    3d74:	8189                	srli	a1,a1,0x2
    3d76:	00159413          	slli	s0,a1,0x1
    3d7a:	025585b3          	mul	a1,a1,t0
    3d7e:	40b705b3          	sub	a1,a4,a1
    3d82:	0586                	slli	a1,a1,0x1
    3d84:	01e5f5b3          	and	a1,a1,t5
    3d88:	01040733          	add	a4,s0,a6
    3d8c:	00d38433          	add	s0,t2,a3
    3d90:	00074483          	lbu	s1,0(a4)
    3d94:	00170703          	lb	a4,1(a4)
    3d98:	95c2                	add	a1,a1,a6
    3d9a:	00158783          	lb	a5,1(a1) # 5f5e001 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x5f4ee91>
    3d9e:	0005c583          	lbu	a1,0(a1)
    3da2:	02e40223          	sb	a4,36(s0)
    3da6:	029401a3          	sb	s1,35(s0)
    3daa:	02f40323          	sb	a5,38(s0)
    3dae:	02b402a3          	sb	a1,37(s0)
    3db2:	16f1                	addi	a3,a3,-4
    3db4:	facfe2e3          	bltu	t6,a2,3d58 <.LBB475_10+0x84>
    3db8:	02768693          	addi	a3,a3,39
    3dbc:	06300593          	li	a1,99
    3dc0:	f325d4e3          	bge	a1,s2,3ce8 <.LBB475_10+0x14>
    3dc4:	65c1                	lui	a1,0x10
    3dc6:	ffc5861b          	addiw	a2,a1,-4
    3dca:	00c97633          	and	a2,s2,a2
    3dce:	8209                	srli	a2,a2,0x2
    3dd0:	0051f737          	lui	a4,0x51f
    3dd4:	b857071b          	addiw	a4,a4,-1147
    3dd8:	0736                	slli	a4,a4,0xd
    3dda:	3d770713          	addi	a4,a4,983 # 51f3d7 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x510267>
    3dde:	073a                	slli	a4,a4,0xe
    3de0:	28f70713          	addi	a4,a4,655
    3de4:	0732                	slli	a4,a4,0xc
    3de6:	5c370713          	addi	a4,a4,1475
    3dea:	02e63633          	mulhu	a2,a2,a4
    3dee:	8209                	srli	a2,a2,0x2
    3df0:	06400713          	li	a4,100
    3df4:	02e60733          	mul	a4,a2,a4
    3df8:	40e90733          	sub	a4,s2,a4
    3dfc:	0706                	slli	a4,a4,0x1
    3dfe:	35f9                	addiw	a1,a1,-2
    3e00:	8df9                	and	a1,a1,a4
    3e02:	16f9                	addi	a3,a3,-2
    3e04:	95c2                	add	a1,a1,a6
    3e06:	00158703          	lb	a4,1(a1) # 10001 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xe91>
    3e0a:	0005c583          	lbu	a1,0(a1)
    3e0e:	00910793          	addi	a5,sp,9
    3e12:	97b6                	add	a5,a5,a3
    3e14:	00e780a3          	sb	a4,1(a5)
    3e18:	00b78023          	sb	a1,0(a5)
    3e1c:	8932                	mv	s2,a2
    3e1e:	45a5                	li	a1,9
    3e20:	ed25d7e3          	bge	a1,s2,3cee <.LBB475_10+0x1a>
    3e24:	00191613          	slli	a2,s2,0x1
    3e28:	ffe68593          	addi	a1,a3,-2
    3e2c:	9642                	add	a2,a2,a6
    3e2e:	00160683          	lb	a3,1(a2)
    3e32:	00064603          	lbu	a2,0(a2)
    3e36:	00910713          	addi	a4,sp,9
    3e3a:	972e                	add	a4,a4,a1
    3e3c:	00d700a3          	sb	a3,1(a4)
    3e40:	00c70023          	sb	a2,0(a4)
    3e44:	00910613          	addi	a2,sp,9
    3e48:	00b60733          	add	a4,a2,a1
    3e4c:	02700613          	li	a2,39
    3e50:	40b607b3          	sub	a5,a2,a1

0000000000003e54 <.LBB475_11>:
    3e54:	00002617          	auipc	a2,0x2
    3e58:	98c60613          	addi	a2,a2,-1652 # 57e0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.134>
    3e5c:	4585                	li	a1,1
    3e5e:	4681                	li	a3,0
    3e60:	fffff097          	auipc	ra,0xfffff
    3e64:	a24080e7          	jalr	-1500(ra) # 2884 <_ZN4core3fmt9Formatter12pad_integral17h62dfae4d5e1cb061E>
    3e68:	7942                	ld	s2,48(sp)
    3e6a:	74e2                	ld	s1,56(sp)
    3e6c:	6406                	ld	s0,64(sp)
    3e6e:	60a6                	ld	ra,72(sp)
    3e70:	6161                	addi	sp,sp,80
    3e72:	8082                	ret

0000000000003e74 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h93049563736a027cE>:
    3e74:	6510                	ld	a2,8(a0)
    3e76:	6108                	ld	a0,0(a0)
    3e78:	6e1c                	ld	a5,24(a2)
    3e7a:	8782                	jr	a5

0000000000003e7c <_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hc6a9db97781cac35E>:
    3e7c:	6114                	ld	a3,0(a0)
    3e7e:	6510                	ld	a2,8(a0)
    3e80:	852e                	mv	a0,a1
    3e82:	85b6                	mv	a1,a3
    3e84:	fffff317          	auipc	t1,0xfffff
    3e88:	c8a30067          	jr	-886(t1) # 2b0e <_ZN4core3fmt9Formatter3pad17h237bfdd037fe5ac0E>

0000000000003e8c <_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h2d824475a5ff264fE>:
    3e8c:	1141                	addi	sp,sp,-16
    3e8e:	e406                	sd	ra,8(sp)
    3e90:	4681                	li	a3,0
    3e92:	0005059b          	sext.w	a1,a0
    3e96:	6645                	lui	a2,0x11
    3e98:	d246071b          	addiw	a4,a2,-732
    3e9c:	00b5161b          	slliw	a2,a0,0xb
    3ea0:	00e5e363          	bltu	a1,a4,3ea6 <_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h2d824475a5ff264fE+0x1a>
    3ea4:	46bd                	li	a3,15
    3ea6:	00868713          	addi	a4,a3,8
    3eaa:	00271793          	slli	a5,a4,0x2

0000000000003eae <.LBB619_29>:
    3eae:	00002597          	auipc	a1,0x2
    3eb2:	4d258593          	addi	a1,a1,1234 # 6380 <_ZN4core7unicode12unicode_data15grapheme_extend17SHORT_OFFSET_RUNS17hff6f6dc47b6092b7E>
    3eb6:	97ae                	add	a5,a5,a1
    3eb8:	439c                	lw	a5,0(a5)
    3eba:	00b7979b          	slliw	a5,a5,0xb
    3ebe:	00f66363          	bltu	a2,a5,3ec4 <.LBB619_29+0x16>
    3ec2:	86ba                	mv	a3,a4
    3ec4:	00468713          	addi	a4,a3,4
    3ec8:	00271793          	slli	a5,a4,0x2
    3ecc:	97ae                	add	a5,a5,a1
    3ece:	439c                	lw	a5,0(a5)
    3ed0:	00b7979b          	slliw	a5,a5,0xb
    3ed4:	00f66363          	bltu	a2,a5,3eda <.LBB619_29+0x2c>
    3ed8:	86ba                	mv	a3,a4
    3eda:	00268713          	addi	a4,a3,2
    3ede:	00271793          	slli	a5,a4,0x2
    3ee2:	97ae                	add	a5,a5,a1
    3ee4:	439c                	lw	a5,0(a5)
    3ee6:	00b7979b          	slliw	a5,a5,0xb
    3eea:	00f66363          	bltu	a2,a5,3ef0 <.LBB619_29+0x42>
    3eee:	86ba                	mv	a3,a4
    3ef0:	00168713          	addi	a4,a3,1
    3ef4:	00271793          	slli	a5,a4,0x2
    3ef8:	97ae                	add	a5,a5,a1
    3efa:	439c                	lw	a5,0(a5)
    3efc:	00b7979b          	slliw	a5,a5,0xb
    3f00:	00f66363          	bltu	a2,a5,3f06 <.LBB619_29+0x58>
    3f04:	86ba                	mv	a3,a4
    3f06:	00269713          	slli	a4,a3,0x2
    3f0a:	972e                	add	a4,a4,a1
    3f0c:	4318                	lw	a4,0(a4)
    3f0e:	00b7171b          	slliw	a4,a4,0xb
    3f12:	00c737b3          	sltu	a5,a4,a2
    3f16:	8e39                	xor	a2,a2,a4
    3f18:	00163613          	seqz	a2,a2
    3f1c:	963e                	add	a2,a2,a5
    3f1e:	00d60733          	add	a4,a2,a3
    3f22:	46f9                	li	a3,30
    3f24:	0ae6e463          	bltu	a3,a4,3fcc <.LBB619_32>
    3f28:	00271793          	slli	a5,a4,0x2
    3f2c:	2b100613          	li	a2,689
    3f30:	00d70763          	beq	a4,a3,3f3e <.LBB619_29+0x90>
    3f34:	00f58633          	add	a2,a1,a5
    3f38:	00466603          	lwu	a2,4(a2) # 11004 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1e94>
    3f3c:	8255                	srli	a2,a2,0x15
    3f3e:	fff70693          	addi	a3,a4,-1
    3f42:	00d77463          	bgeu	a4,a3,3f4a <.LBB619_29+0x9c>
    3f46:	4701                	li	a4,0
    3f48:	a821                	j	3f60 <.LBB619_29+0xb2>
    3f4a:	477d                	li	a4,31
    3f4c:	08e6fb63          	bgeu	a3,a4,3fe2 <.LBB619_33>
    3f50:	068a                	slli	a3,a3,0x2
    3f52:	96ae                	add	a3,a3,a1
    3f54:	0006e683          	lwu	a3,0(a3)
    3f58:	00200737          	lui	a4,0x200
    3f5c:	377d                	addiw	a4,a4,-1
    3f5e:	8f75                	and	a4,a4,a3
    3f60:	95be                	add	a1,a1,a5
    3f62:	0005e583          	lwu	a1,0(a1)
    3f66:	81d5                	srli	a1,a1,0x15
    3f68:	00158693          	addi	a3,a1,1
    3f6c:	02d60f63          	beq	a2,a3,3faa <.LBB619_30+0x22>
    3f70:	2b100793          	li	a5,689
    3f74:	882e                	mv	a6,a1
    3f76:	00b7e463          	bltu	a5,a1,3f7e <.LBB619_29+0xd0>
    3f7a:	2b100813          	li	a6,689
    3f7e:	4781                	li	a5,0
    3f80:	40e5073b          	subw	a4,a0,a4
    3f84:	fff60513          	addi	a0,a2,-1

0000000000003f88 <.LBB619_30>:
    3f88:	00002617          	auipc	a2,0x2
    3f8c:	47460613          	addi	a2,a2,1140 # 63fc <_ZN4core7unicode12unicode_data15grapheme_extend7OFFSETS17h21859fdd3179f732E>
    3f90:	02b80263          	beq	a6,a1,3fb4 <.LBB619_31>
    3f94:	00c586b3          	add	a3,a1,a2
    3f98:	0006c683          	lbu	a3,0(a3)
    3f9c:	9fb5                	addw	a5,a5,a3
    3f9e:	00f76663          	bltu	a4,a5,3faa <.LBB619_30+0x22>
    3fa2:	0585                	addi	a1,a1,1
    3fa4:	feb516e3          	bne	a0,a1,3f90 <.LBB619_30+0x8>
    3fa8:	85aa                	mv	a1,a0
    3faa:	0015f513          	andi	a0,a1,1
    3fae:	60a2                	ld	ra,8(sp)
    3fb0:	0141                	addi	sp,sp,16
    3fb2:	8082                	ret

0000000000003fb4 <.LBB619_31>:
    3fb4:	00002617          	auipc	a2,0x2
    3fb8:	39c60613          	addi	a2,a2,924 # 6350 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.358>
    3fbc:	2b100593          	li	a1,689
    3fc0:	8542                	mv	a0,a6
    3fc2:	ffffe097          	auipc	ra,0xffffe
    3fc6:	fac080e7          	jalr	-84(ra) # 1f6e <_ZN4core9panicking18panic_bounds_check17hb80951707fb532bdE>
	...

0000000000003fcc <.LBB619_32>:
    3fcc:	00002617          	auipc	a2,0x2
    3fd0:	36c60613          	addi	a2,a2,876 # 6338 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.357>
    3fd4:	45fd                	li	a1,31
    3fd6:	853a                	mv	a0,a4
    3fd8:	ffffe097          	auipc	ra,0xffffe
    3fdc:	f96080e7          	jalr	-106(ra) # 1f6e <_ZN4core9panicking18panic_bounds_check17hb80951707fb532bdE>
	...

0000000000003fe2 <.LBB619_33>:
    3fe2:	00002617          	auipc	a2,0x2
    3fe6:	38660613          	addi	a2,a2,902 # 6368 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.359>
    3fea:	45fd                	li	a1,31
    3fec:	8536                	mv	a0,a3
    3fee:	ffffe097          	auipc	ra,0xffffe
    3ff2:	f80080e7          	jalr	-128(ra) # 1f6e <_ZN4core9panicking18panic_bounds_check17hb80951707fb532bdE>
	...

0000000000003ff8 <memcpy>:
    3ff8:	ca11                	beqz	a2,400c <memcpy+0x14>
    3ffa:	86aa                	mv	a3,a0
    3ffc:	00058703          	lb	a4,0(a1)
    4000:	00e68023          	sb	a4,0(a3)
    4004:	167d                	addi	a2,a2,-1
    4006:	0685                	addi	a3,a3,1
    4008:	0585                	addi	a1,a1,1
    400a:	fa6d                	bnez	a2,3ffc <memcpy+0x4>
    400c:	8082                	ret

000000000000400e <memset>:
    400e:	c619                	beqz	a2,401c <memset+0xe>
    4010:	86aa                	mv	a3,a0
    4012:	00b68023          	sb	a1,0(a3)
    4016:	167d                	addi	a2,a2,-1
    4018:	0685                	addi	a3,a3,1
    401a:	fe65                	bnez	a2,4012 <memset+0x4>
    401c:	8082                	ret

000000000000401e <bcmp>:
    401e:	ca19                	beqz	a2,4034 <bcmp+0x16>
    4020:	00054683          	lbu	a3,0(a0)
    4024:	0005c703          	lbu	a4,0(a1)
    4028:	00e69863          	bne	a3,a4,4038 <bcmp+0x1a>
    402c:	167d                	addi	a2,a2,-1
    402e:	0585                	addi	a1,a1,1
    4030:	0505                	addi	a0,a0,1
    4032:	f67d                	bnez	a2,4020 <bcmp+0x2>
    4034:	4501                	li	a0,0
    4036:	8082                	ret
    4038:	40e68533          	sub	a0,a3,a4
    403c:	8082                	ret

Disassembly of section .rodata:

0000000000005000 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.0>:
    5000:	6f74                	ld	a3,216(a4)
    5002:	6e72                	ld	t3,280(sp)
    5004:	6461                	lui	s0,0x18
    5006:	73752d6f          	jal	s10,57f3c <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x48dcc>
    500a:	7265                	lui	tp,0xffff9
    500c:	6372732f          	0x6372732f
    5010:	6e69622f          	0x6e69622f
    5014:	6573752f          	0x6573752f
    5018:	5f72                	lw	t5,60(sp)
    501a:	6174                	ld	a3,192(a0)
    501c:	722e6b73          	csrrsi	s6,0x722,28
    5020:	00000073          	ecall
    5024:	0000                	unimp
	...

0000000000005028 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.1>:
    5028:	5000                	lw	s0,32(s0)
    502a:	0000                	unimp
    502c:	0000                	unimp
    502e:	0000                	unimp
    5030:	0021                	c.nop	8
    5032:	0000                	unimp
    5034:	0000                	unimp
    5036:	0000                	unimp
    5038:	0018                	0x18
    503a:	0000                	unimp
    503c:	0005                	c.nop	1
    503e:	0000                	unimp
    5040:	2029                	0x2029
    5042:	6e656877          	0x6e656877
    5046:	7320                	ld	s0,96(a4)
    5048:	696c                	ld	a1,208(a0)
    504a:	676e6963          	bltu	t3,s6,56bc <.Lanon.9ecd2340286333da0be21283f7862ea4.2+0xc>
    504e:	6020                	ld	s0,64(s0)
    5050:	0001                	nop
    5052:	0000                	unimp
    5054:	0000                	unimp
    5056:	0000                	unimp
    5058:	0008                	0x8
    505a:	0000                	unimp
    505c:	0000                	unimp
    505e:	0000                	unimp
    5060:	6172                	ld	sp,280(sp)
    5062:	676e                	ld	a4,216(sp)
    5064:	2065                	0x2065
    5066:	6e65                	lui	t3,0x19
    5068:	2064                	fld	fs1,192(s0)
    506a:	6e69                	lui	t3,0x1a
    506c:	6564                	ld	s1,200(a0)
    506e:	2078                	fld	fa4,192(s0)

0000000000005070 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.3>:
    5070:	5000                	lw	s0,32(s0)
    5072:	0000                	unimp
    5074:	0000                	unimp
    5076:	0000                	unimp
    5078:	0021                	c.nop	8
    507a:	0000                	unimp
    507c:	0000                	unimp
    507e:	0000                	unimp
    5080:	001d                	c.nop	7
    5082:	0000                	unimp
    5084:	0005                	c.nop	1
	...

0000000000005088 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.4>:
    5088:	5000                	lw	s0,32(s0)
    508a:	0000                	unimp
    508c:	0000                	unimp
    508e:	0000                	unimp
    5090:	0021                	c.nop	8
    5092:	0000                	unimp
    5094:	0000                	unimp
    5096:	0000                	unimp
    5098:	003d                	c.nop	15
    509a:	0000                	unimp
    509c:	0005                	c.nop	1
	...

00000000000050a0 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.5>:
    50a0:	6e69                	lui	t3,0x1a
    50a2:	6574                	ld	a3,200(a0)
    50a4:	6e72                	ld	t3,280(sp)
    50a6:	6c61                	lui	s8,0x18
    50a8:	6520                	ld	s0,72(a0)
    50aa:	7272                	ld	tp,312(sp)
    50ac:	203a726f          	jal	tp,acaae <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x9d93e>
    50b0:	6e65                	lui	t3,0x19
    50b2:	6574                	ld	a3,200(a0)
    50b4:	6572                	ld	a0,280(sp)
    50b6:	2064                	fld	fs1,192(s0)
    50b8:	6e75                	lui	t3,0x1d
    50ba:	6572                	ld	a0,280(sp)
    50bc:	6361                	lui	t1,0x18
    50be:	6168                	ld	a0,192(a0)
    50c0:	6c62                	ld	s8,24(sp)
    50c2:	2065                	0x2065
    50c4:	65646f63          	bltu	s0,s6,5722 <.Lanon.9ecd2340286333da0be21283f7862ea4.7+0x12>

00000000000050c8 <.Lanon.e31c4ceddddbe0e076f3de1d2d2eda96.6>:
    50c8:	5000                	lw	s0,32(s0)
    50ca:	0000                	unimp
    50cc:	0000                	unimp
    50ce:	0000                	unimp
    50d0:	0021                	c.nop	8
    50d2:	0000                	unimp
    50d4:	0000                	unimp
    50d6:	0000                	unimp
    50d8:	0040                	addi	s0,sp,4
    50da:	0000                	unimp
    50dc:	0005                	c.nop	1
	...

00000000000050e0 <anon.8e5db402a9e16f5ceff60bed8bcd7dec.0.llvm.14341049042176442005>:
    50e0:	032a                	slli	t1,t1,0xa
    50e2:	0000                	unimp
    50e4:	0000                	unimp
    50e6:	0000                	unimp
    50e8:	0008                	0x8
    50ea:	0000                	unimp
    50ec:	0000                	unimp
    50ee:	0000                	unimp
    50f0:	0008                	0x8
    50f2:	0000                	unimp
    50f4:	0000                	unimp
    50f6:	0000                	unimp
    50f8:	0492                	slli	s1,s1,0x4
    50fa:	0000                	unimp
    50fc:	0000                	unimp
	...

0000000000005100 <anon.ac1ba77cd7d7837b7067d1be6c826dee.0.llvm.1899440800677633648>:
    5100:	0430                	addi	a2,sp,520
    5102:	0000                	unimp
    5104:	0000                	unimp
    5106:	0000                	unimp
    5108:	0020                	addi	s0,sp,8
    510a:	0000                	unimp
    510c:	0000                	unimp
    510e:	0000                	unimp
    5110:	0008                	0x8
    5112:	0000                	unimp
    5114:	0000                	unimp
    5116:	0000                	unimp
    5118:	0288                	addi	a0,sp,320
    511a:	0000                	unimp
    511c:	0000                	unimp
	...

0000000000005120 <.Lanon.2cd93b564825415fcd61733825743066.0>:
    5120:	7361                	lui	t1,0xffff8
    5122:	74726573          	csrrsi	a0,0x747,4
    5126:	6f69                	lui	t5,0x1a
    5128:	206e                	fld	ft0,216(sp)
    512a:	6166                	ld	sp,88(sp)
    512c:	6c69                	lui	s8,0x1a
    512e:	6465                	lui	s0,0x19
    5130:	203a                	fld	ft0,392(sp)
    5132:	666c6573          	csrrsi	a0,0x666,24
    5136:	632e                	ld	t1,200(sp)
    5138:	7061                	c.lui	zero,0xffff8
    513a:	2928                	fld	fa0,80(a0)
    513c:	3d20                	fld	fs0,120(a0)
    513e:	203d                	0x203d
    5140:	5f646c6f          	jal	s8,4b736 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x3c5c6>
    5144:	20706163          	bltu	zero,t2,5346 <anon.94a30639c8e43a3f584e10ab8b53be30.1.llvm.1569364017202141003+0x2b>
    5148:	202a                	fld	ft0,136(sp)
    514a:	                	fld	ft10,264(sp)

000000000000514b <.Lanon.2cd93b564825415fcd61733825743066.1>:
    514b:	7375722f          	0x7375722f
    514f:	6374                	ld	a3,192(a4)
    5151:	3733652f          	0x3733652f
    5155:	3161                	addiw	sp,sp,-8
    5157:	33636333          	0x33636333
    515b:	3935                	addiw	s2,s2,-19
    515d:	3034                	fld	fa3,96(s0)
    515f:	3430                	fld	fa2,104(s0)
    5161:	3636                	fld	fa2,360(sp)
    5163:	38333733          	0x38333733
    5167:	6462                	ld	s0,24(sp)
    5169:	3831                	addiw	a6,a6,-20
    516b:	3864                	fld	fs1,240(s0)
    516d:	3031                	0x3031
    516f:	6530                	ld	a2,72(a0)
    5171:	6436                	ld	s0,328(sp)
    5173:	3962                	fld	fs2,56(sp)
    5175:	3636                	fld	fa2,360(sp)
    5177:	6336                	ld	t1,328(sp)
    5179:	2f66                	fld	ft10,88(sp)
    517b:	696c                	ld	a1,208(a0)
    517d:	7262                	ld	tp,56(sp)
    517f:	7261                	lui	tp,0xffff8
    5181:	2f79                	addiw	t5,t5,30
    5183:	6c61                	lui	s8,0x18
    5185:	6f6c                	ld	a1,216(a4)
    5187:	72732f63          	0x72732f63
    518b:	6f632f63          	0x6f632f63
    518f:	6c6c                	ld	a1,216(s0)
    5191:	6365                	lui	t1,0x19
    5193:	6974                	ld	a3,208(a0)
    5195:	2f736e6f          	jal	t3,3bc8b <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x2cb1b>
    5199:	6576                	ld	a0,344(sp)
    519b:	65645f63          	bge	s0,s6,57f9 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.169+0x1>
    519f:	7571                	lui	a0,0xffffc
    51a1:	2f65                	addiw	t5,t5,25
    51a3:	6f6d                	lui	t5,0x1b
    51a5:	2e64                	fld	fs1,216(a2)
    51a7:	7372                	ld	t1,312(sp)
    51a9:	0000                	unimp
    51ab:	0000                	unimp
    51ad:	0000                	unimp
	...

00000000000051b0 <.Lanon.2cd93b564825415fcd61733825743066.2>:
    51b0:	0000514b          	fnmsub.s	ft2,ft0,ft0,ft0,unknown
    51b4:	0000                	unimp
    51b6:	0000                	unimp
    51b8:	005e                	c.slli	zero,0x17
    51ba:	0000                	unimp
    51bc:	0000                	unimp
    51be:	0000                	unimp
    51c0:	082d                	addi	a6,a6,11
    51c2:	0000                	unimp
    51c4:	000d                	c.nop	3
	...

00000000000051c8 <anon.be5ea868b286bb36e8d2b81fd8475abb.0.llvm.4157917051624895175>:
    51c8:	0860                	addi	s0,sp,28
    51ca:	0000                	unimp
    51cc:	0000                	unimp
    51ce:	0000                	unimp
    51d0:	082a                	slli	a6,a6,0xa
    51d2:	0000                	unimp
    51d4:	0000                	unimp
    51d6:	0000                	unimp
    51d8:	087e                	slli	a6,a6,0x1f
    51da:	0000                	unimp
    51dc:	0000                	unimp
    51de:	0000                	unimp
    51e0:	0800                	addi	s0,sp,16
    51e2:	0000                	unimp
    51e4:	0000                	unimp
    51e6:	0000                	unimp
    51e8:	2020                	fld	fs0,64(s0)
    51ea:	2020                	fld	fs0,64(s0)
    51ec:	656d6f53          	0x656d6f53
    51f0:	3c20                	fld	fs0,120(s0)
    51f2:	203d                	0x203d
    51f4:	6f4e                	ld	t5,208(sp)
    51f6:	656e                	ld	a0,216(sp)

00000000000051f8 <.Lanon.a5be637f67c858a9fd1543754584150a.1>:
    51f8:	0904                	addi	s1,sp,144
    51fa:	0000                	unimp
    51fc:	0000                	unimp
    51fe:	0000                	unimp
    5200:	0008                	0x8
    5202:	0000                	unimp
    5204:	0000                	unimp
    5206:	0000                	unimp
    5208:	0008                	0x8
    520a:	0000                	unimp
    520c:	0000                	unimp
    520e:	0000                	unimp
    5210:	0432                	slli	s0,s0,0xc
    5212:	0000                	unimp
    5214:	0000                	unimp
	...

0000000000005218 <anon.780e3732ec4a8f2c2a9317f7c3bfa294.0.llvm.1894474860202936077>:
    5218:	0906                	slli	s2,s2,0x1
    521a:	0000                	unimp
    521c:	0000                	unimp
    521e:	0000                	unimp
    5220:	0008                	0x8
    5222:	0000                	unimp
    5224:	0000                	unimp
    5226:	0000                	unimp
    5228:	0008                	0x8
    522a:	0000                	unimp
    522c:	0000                	unimp
    522e:	0000                	unimp
    5230:	0896                	slli	a7,a7,0x5
    5232:	0000                	unimp
    5234:	0000                	unimp
	...

0000000000005238 <anon.780e3732ec4a8f2c2a9317f7c3bfa294.1.llvm.1894474860202936077>:
    5238:	0906                	slli	s2,s2,0x1
    523a:	0000                	unimp
    523c:	0000                	unimp
    523e:	0000                	unimp
    5240:	0008                	0x8
    5242:	0000                	unimp
    5244:	0000                	unimp
    5246:	0000                	unimp
    5248:	0008                	0x8
    524a:	0000                	unimp
    524c:	0000                	unimp
    524e:	0000                	unimp
    5250:	02be                	slli	t0,t0,0xf
    5252:	0000                	unimp
    5254:	0000                	unimp
	...

0000000000005258 <anon.12908af6f985cbf1ec18d41b8c469eb6.0.llvm.10739002475286810050>:
    5258:	0b00                	addi	s0,sp,400
    525a:	0000                	unimp
    525c:	0000                	unimp
    525e:	0000                	unimp
    5260:	0020                	addi	s0,sp,8
    5262:	0000                	unimp
    5264:	0000                	unimp
    5266:	0000                	unimp
    5268:	0008                	0x8
    526a:	0000                	unimp
    526c:	0000                	unimp
    526e:	0000                	unimp
    5270:	0288                	addi	a0,sp,320
    5272:	0000                	unimp
    5274:	0000                	unimp
	...

0000000000005278 <.Lanon.5ffa1a0ec8fe4778f9c8f909c00eaf94.0>:
    5278:	6c6c6163          	bltu	s8,t1,593a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.190+0x2>
    527c:	6465                	lui	s0,0x19
    527e:	6020                	ld	s0,64(s0)
    5280:	6974704f          	fnmadd.s	ft0,fs0,fs7,fa3
    5284:	3a3a6e6f          	jal	t3,abe26 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x9ccb6>
    5288:	6e75                	lui	t3,0x1d
    528a:	70617277          	0x70617277
    528e:	2928                	fld	fa0,80(a0)
    5290:	2060                	fld	fs0,192(s0)
    5292:	61206e6f          	jal	t3,b8a4 <_ZN12tornado_user10HEAP_SPACE17hcda5f67e49da29e7E+0x486c>
    5296:	6020                	ld	s0,64(s0)
    5298:	6f4e                	ld	t5,208(sp)
    529a:	656e                	ld	a0,216(sp)
    529c:	2060                	fld	fs0,192(s0)
    529e:	6176                	ld	sp,344(sp)
    52a0:	756c                	ld	a1,232(a0)
    52a2:	                	lui	s0,0xffff9

00000000000052a3 <.Lanon.5ffa1a0ec8fe4778f9c8f909c00eaf94.1>:
    52a3:	6f74                	ld	a3,216(a4)
    52a5:	6e72                	ld	t3,280(sp)
    52a7:	6461                	lui	s0,0x18
    52a9:	73752d6f          	jal	s10,581df <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x4906f>
    52ad:	7265                	lui	tp,0xffff9
    52af:	6372732f          	0x6372732f
    52b3:	6378652f          	0x6378652f
    52b7:	7475                	lui	s0,0xffffd
    52b9:	722e726f          	jal	tp,ec9db <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xdd86b>
    52bd:	          	0xa3000073

00000000000052c0 <.Lanon.5ffa1a0ec8fe4778f9c8f909c00eaf94.2>:
    52c0:	000052a3          	0x52a3
    52c4:	0000                	unimp
    52c6:	0000                	unimp
    52c8:	0000001b          	sext.w	zero,zero
    52cc:	0000                	unimp
    52ce:	0000                	unimp
    52d0:	003e                	c.slli	zero,0xf
    52d2:	0000                	unimp
    52d4:	0000002b          	0x2b

00000000000052d8 <_ZN12tornado_user7excutor8EXECUTOR17hf6915d4efcbd1205E>:
    52d8:	0f88                	addi	a0,sp,976
    52da:	0000                	unimp
    52dc:	0000                	unimp
    52de:	0000                	unimp
    52e0:	0eda                	slli	t4,t4,0x16
    52e2:	0000                	unimp
    52e4:	0000                	unimp
    52e6:	0000                	unimp
    52e8:	0fa6                	slli	t6,t6,0x9
    52ea:	0000                	unimp
    52ec:	0000                	unimp
    52ee:	0000                	unimp
    52f0:	0eb0                	addi	a2,sp,856
    52f2:	0000                	unimp
    52f4:	0000                	unimp
	...

00000000000052f8 <anon.94a30639c8e43a3f584e10ab8b53be30.0.llvm.1569364017202141003>:
    52f8:	7361                	lui	t1,0xffff8
    52fa:	74726573          	csrrsi	a0,0x747,4
    52fe:	6f69                	lui	t5,0x1a
    5300:	206e                	fld	ft0,216(sp)
    5302:	6166                	ld	sp,88(sp)
    5304:	6c69                	lui	s8,0x1a
    5306:	6465                	lui	s0,0x19
    5308:	203a                	fld	ft0,392(sp)
    530a:	696d                	lui	s2,0x1b
    530c:	2064                	fld	fs1,192(s0)
    530e:	3d3c                	fld	fa5,120(a0)
    5310:	7320                	ld	s0,96(a4)
    5312:	6c65                	lui	s8,0x19
    5314:	2e66                	fld	ft8,88(sp)
    5316:	656c                	ld	a1,200(a0)
    5318:	286e                	fld	fa6,216(sp)
    531a:	                	addiw	t5,t5,10

000000000000531b <anon.94a30639c8e43a3f584e10ab8b53be30.1.llvm.1569364017202141003>:
    531b:	7375722f          	0x7375722f
    531f:	6374                	ld	a3,192(a4)
    5321:	3733652f          	0x3733652f
    5325:	3161                	addiw	sp,sp,-8
    5327:	33636333          	0x33636333
    532b:	3935                	addiw	s2,s2,-19
    532d:	3034                	fld	fa3,96(s0)
    532f:	3430                	fld	fa2,104(s0)
    5331:	3636                	fld	fa2,360(sp)
    5333:	38333733          	0x38333733
    5337:	6462                	ld	s0,24(sp)
    5339:	3831                	addiw	a6,a6,-20
    533b:	3864                	fld	fs1,240(s0)
    533d:	3031                	0x3031
    533f:	6530                	ld	a2,72(a0)
    5341:	6436                	ld	s0,328(sp)
    5343:	3962                	fld	fs2,56(sp)
    5345:	3636                	fld	fa2,360(sp)
    5347:	6336                	ld	t1,328(sp)
    5349:	2f66                	fld	ft10,88(sp)
    534b:	696c                	ld	a1,208(a0)
    534d:	7262                	ld	tp,56(sp)
    534f:	7261                	lui	tp,0xffff8
    5351:	2f79                	addiw	t5,t5,30
    5353:	65726f63          	bltu	tp,s7,59b1 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.191+0x29>
    5357:	6372732f          	0x6372732f
    535b:	696c732f          	0x696c732f
    535f:	6d2f6563          	bltu	t5,s2,5a29 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.198+0x1>
    5363:	722e646f          	jal	s0,eba85 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xdc915>
    5367:	          	csrrw	s6,utvec,t1

0000000000005368 <anon.94a30639c8e43a3f584e10ab8b53be30.2.llvm.1569364017202141003>:
    5368:	0000531b          	srliw	t1,zero,0x0
    536c:	0000                	unimp
    536e:	0000                	unimp
    5370:	004d                	c.nop	19
    5372:	0000                	unimp
    5374:	0000                	unimp
    5376:	0000                	unimp
    5378:	05ff                	0x5ff
    537a:	0000                	unimp
    537c:	0009                	c.nop	2
	...

0000000000005380 <anon.94a30639c8e43a3f584e10ab8b53be30.3.llvm.1569364017202141003>:
    5380:	7375722f          	0x7375722f
    5384:	6374                	ld	a3,192(a4)
    5386:	3733652f          	0x3733652f
    538a:	3161                	addiw	sp,sp,-8
    538c:	33636333          	0x33636333
    5390:	3935                	addiw	s2,s2,-19
    5392:	3034                	fld	fa3,96(s0)
    5394:	3430                	fld	fa2,104(s0)
    5396:	3636                	fld	fa2,360(sp)
    5398:	38333733          	0x38333733
    539c:	6462                	ld	s0,24(sp)
    539e:	3831                	addiw	a6,a6,-20
    53a0:	3864                	fld	fs1,240(s0)
    53a2:	3031                	0x3031
    53a4:	6530                	ld	a2,72(a0)
    53a6:	6436                	ld	s0,328(sp)
    53a8:	3962                	fld	fs2,56(sp)
    53aa:	3636                	fld	fa2,360(sp)
    53ac:	6336                	ld	t1,328(sp)
    53ae:	2f66                	fld	ft10,88(sp)
    53b0:	696c                	ld	a1,208(a0)
    53b2:	7262                	ld	tp,56(sp)
    53b4:	7261                	lui	tp,0xffff8
    53b6:	2f79                	addiw	t5,t5,30
    53b8:	6c61                	lui	s8,0x18
    53ba:	6f6c                	ld	a1,216(a4)
    53bc:	72732f63          	0x72732f63
    53c0:	6f632f63          	0x6f632f63
    53c4:	6c6c                	ld	a1,216(s0)
    53c6:	6365                	lui	t1,0x19
    53c8:	6974                	ld	a3,208(a0)
    53ca:	2f736e6f          	jal	t3,3bec0 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x2cd50>
    53ce:	6576                	ld	a0,344(sp)
    53d0:	65645f63          	bge	s0,s6,5a2e <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.207>
    53d4:	7571                	lui	a0,0xffffc
    53d6:	2f65                	addiw	t5,t5,25
    53d8:	6972                	ld	s2,280(sp)
    53da:	676e                	ld	a4,216(sp)
    53dc:	735f 696c 6563      	0x6563696c735f
    53e2:	73722e73          	csrrs	t3,0x737,tp
	...

00000000000053e8 <anon.94a30639c8e43a3f584e10ab8b53be30.4.llvm.1569364017202141003>:
    53e8:	5380                	lw	s0,32(a5)
    53ea:	0000                	unimp
    53ec:	0000                	unimp
    53ee:	0000                	unimp
    53f0:	0066                	c.slli	zero,0x19
    53f2:	0000                	unimp
    53f4:	0000                	unimp
    53f6:	0000                	unimp
    53f8:	0020                	addi	s0,sp,8
    53fa:	0000                	unimp
    53fc:	000e                	c.slli	zero,0x3
	...

0000000000005400 <.Lanon.a6cd8df4e24dac0f0dc7ecbcbd5a7bc0.0>:
    5400:	6f74                	ld	a3,216(a4)
    5402:	616d206f          	j	d7a18 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xc88a8>
    5406:	796e                	ld	s2,248(sp)
    5408:	7420                	ld	s0,104(s0)
    540a:	7361                	lui	t1,0xffff8
    540c:	          	0x7421736b

000000000000540f <.Lanon.a6cd8df4e24dac0f0dc7ecbcbd5a7bc0.1>:
    540f:	6f74                	ld	a3,216(a4)
    5411:	6e72                	ld	t3,280(sp)
    5413:	6461                	lui	s0,0x18
    5415:	73752d6f          	jal	s10,5834b <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x491db>
    5419:	7265                	lui	tp,0xffff9
    541b:	6372732f          	0x6372732f
    541f:	7361742f          	0x7361742f
    5423:	73722e6b          	0x73722e6b
	...

0000000000005428 <.Lanon.a6cd8df4e24dac0f0dc7ecbcbd5a7bc0.2>:
    5428:	0000540f          	0x540f
    542c:	0000                	unimp
    542e:	0000                	unimp
    5430:	0018                	0x18
    5432:	0000                	unimp
    5434:	0000                	unimp
    5436:	0000                	unimp
    5438:	00000033          	add	zero,zero,zero
    543c:	000d                	c.nop	3
	...

0000000000005440 <.Lanon.24b97dc0e08433fc2bb4290f4e5e47d5.0>:
    5440:	7375722f          	0x7375722f
    5444:	6374                	ld	a3,192(a4)
    5446:	3733652f          	0x3733652f
    544a:	3161                	addiw	sp,sp,-8
    544c:	33636333          	0x33636333
    5450:	3935                	addiw	s2,s2,-19
    5452:	3034                	fld	fa3,96(s0)
    5454:	3430                	fld	fa2,104(s0)
    5456:	3636                	fld	fa2,360(sp)
    5458:	38333733          	0x38333733
    545c:	6462                	ld	s0,24(sp)
    545e:	3831                	addiw	a6,a6,-20
    5460:	3864                	fld	fs1,240(s0)
    5462:	3031                	0x3031
    5464:	6530                	ld	a2,72(a0)
    5466:	6436                	ld	s0,328(sp)
    5468:	3962                	fld	fs2,56(sp)
    546a:	3636                	fld	fa2,360(sp)
    546c:	6336                	ld	t1,328(sp)
    546e:	2f66                	fld	ft10,88(sp)
    5470:	696c                	ld	a1,208(a0)
    5472:	7262                	ld	tp,56(sp)
    5474:	7261                	lui	tp,0xffff8
    5476:	2f79                	addiw	t5,t5,30
    5478:	6c61                	lui	s8,0x18
    547a:	6f6c                	ld	a1,216(a4)
    547c:	72732f63          	0x72732f63
    5480:	6f632f63          	0x6f632f63
    5484:	6c6c                	ld	a1,216(s0)
    5486:	6365                	lui	t1,0x19
    5488:	6974                	ld	a3,208(a0)
    548a:	2f736e6f          	jal	t3,3bf80 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x2ce10>
    548e:	6576                	ld	a0,344(sp)
    5490:	65645f63          	bge	s0,s6,5aee <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.228+0x84>
    5494:	7571                	lui	a0,0xffffc
    5496:	2f65                	addiw	t5,t5,25
    5498:	6f6d                	lui	t5,0x1b
    549a:	2e64                	fld	fs1,216(a2)
    549c:	7372                	ld	t1,312(sp)

000000000000549e <.Lanon.24b97dc0e08433fc2bb4290f4e5e47d5.1>:
    549e:	7361                	lui	t1,0xffff8
    54a0:	74726573          	csrrsi	a0,0x747,4
    54a4:	6f69                	lui	t5,0x1a
    54a6:	206e                	fld	ft0,216(sp)
    54a8:	6166                	ld	sp,88(sp)
    54aa:	6c69                	lui	s8,0x1a
    54ac:	6465                	lui	s0,0x19
    54ae:	203a                	fld	ft0,392(sp)
    54b0:	666c6573          	csrrsi	a0,0x666,24
    54b4:	632e                	ld	t1,200(sp)
    54b6:	7061                	c.lui	zero,0xffff8
    54b8:	2928                	fld	fa0,80(a0)
    54ba:	3d20                	fld	fs0,120(a0)
    54bc:	203d                	0x203d
    54be:	5f646c6f          	jal	s8,4bab4 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x3c944>
    54c2:	20706163          	bltu	zero,t2,56c4 <.Lanon.9ecd2340286333da0be21283f7862ea4.2+0x14>
    54c6:	202a                	fld	ft0,136(sp)
    54c8:	0032                	c.slli	zero,0xc
    54ca:	0000                	unimp
    54cc:	0000                	unimp
	...

00000000000054d0 <.Lanon.24b97dc0e08433fc2bb4290f4e5e47d5.2>:
    54d0:	5440                	lw	s0,44(s0)
    54d2:	0000                	unimp
    54d4:	0000                	unimp
    54d6:	0000                	unimp
    54d8:	005e                	c.slli	zero,0x17
    54da:	0000                	unimp
    54dc:	0000                	unimp
    54de:	0000                	unimp
    54e0:	082d                	addi	a6,a6,11
    54e2:	0000                	unimp
    54e4:	000d                	c.nop	3
	...

00000000000054e8 <.Lanon.f915c59a07edaa678460ac0d37a0bb80.0>:
    54e8:	6e69                	lui	t3,0x1a
    54ea:	6574                	ld	a3,200(a0)
    54ec:	6e72                	ld	t3,280(sp)
    54ee:	6c61                	lui	s8,0x18
    54f0:	6520                	ld	s0,72(a0)
    54f2:	7272                	ld	tp,312(sp)
    54f4:	203a726f          	jal	tp,acef6 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x9dd86>
    54f8:	6e65                	lui	t3,0x19
    54fa:	6574                	ld	a3,200(a0)
    54fc:	6572                	ld	a0,280(sp)
    54fe:	2064                	fld	fs1,192(s0)
    5500:	6e75                	lui	t3,0x1d
    5502:	6572                	ld	a0,280(sp)
    5504:	6361                	lui	t1,0x18
    5506:	6168                	ld	a0,192(a0)
    5508:	6c62                	ld	s8,24(sp)
    550a:	2065                	0x2065
    550c:	65646f63          	bltu	s0,s6,5b6a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.245+0x2>

0000000000005510 <.Lanon.f915c59a07edaa678460ac0d37a0bb80.1>:
    5510:	6f74                	ld	a3,216(a4)
    5512:	6e72                	ld	t3,280(sp)
    5514:	6461                	lui	s0,0x18
    5516:	73752d6f          	jal	s10,5844c <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x492dc>
    551a:	7265                	lui	tp,0xffff9
    551c:	6372732f          	0x6372732f
    5520:	62696c2f          	0x62696c2f
    5524:	722e                	ld	tp,232(sp)
    5526:	          	0x55100073

0000000000005528 <.Lanon.f915c59a07edaa678460ac0d37a0bb80.2>:
    5528:	5510                	lw	a2,40(a0)
    552a:	0000                	unimp
    552c:	0000                	unimp
    552e:	0000                	unimp
    5530:	00000017          	auipc	zero,0x0
    5534:	0000                	unimp
    5536:	0000                	unimp
    5538:	001f 0000 0005      	0x50000001f
	...

0000000000005540 <.Lanon.f915c59a07edaa678460ac0d37a0bb80.3>:
    5540:	5510                	lw	a2,40(a0)
    5542:	0000                	unimp
    5544:	0000                	unimp
    5546:	0000                	unimp
    5548:	00000017          	auipc	zero,0x0
    554c:	0000                	unimp
    554e:	0000                	unimp
    5550:	0026                	c.slli	zero,0x9
    5552:	0000                	unimp
    5554:	0005                	c.nop	1
	...

0000000000005558 <anon.d80fe1a93814aa1e9fa97fc113a24de8.0.llvm.13337655799424603394>:
    5558:	65636e4f          	0x65636e4f
    555c:	6820                	ld	s0,80(s0)
    555e:	7361                	lui	t1,0xffff8
    5560:	7020                	ld	s0,96(s0)
    5562:	6e61                	lui	t3,0x18
    5564:	6369                	lui	t1,0x1a
    5566:	          	0x2f64656b

0000000000005569 <anon.d80fe1a93814aa1e9fa97fc113a24de8.1.llvm.13337655799424603394>:
    5569:	6d6f682f          	0x6d6f682f
    556d:	2f65                	addiw	t5,t5,25
    556f:	7568                	ld	a0,232(a0)
    5571:	63637473          	csrrci	s0,0x636,6
    5575:	632e2f63          	0x632e2f63
    5579:	7261                	lui	tp,0xffff8
    557b:	722f6f67          	0x722f6f67
    557f:	6765                	lui	a4,0x19
    5581:	7369                	lui	t1,0xffffa
    5583:	7274                	ld	a3,224(a2)
    5585:	2f79                	addiw	t5,t5,30
    5587:	2f637273          	csrrci	tp,0x2f6,6
    558b:	696d                	lui	s2,0x1b
    558d:	7272                	ld	tp,312(sp)
    558f:	2e73726f          	jal	tp,3d075 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x2df05>
    5593:	7375                	lui	t1,0xffffd
    5595:	6374                	ld	a3,192(a4)
    5597:	652e                	ld	a0,200(sp)
    5599:	7564                	ld	s1,232(a0)
    559b:	632e                	ld	t1,200(sp)
    559d:	2d6e                	fld	fs10,216(sp)
    559f:	3136                	fld	ft2,360(sp)
    55a1:	6665                	lui	a2,0x19
    55a3:	6536                	ld	a0,328(sp)
    55a5:	6330                	ld	a2,64(a4)
    55a7:	3064                	fld	fs1,224(s0)
    55a9:	6636                	ld	a2,328(sp)
    55ab:	3962                	fld	fs2,56(sp)
    55ad:	3862                	fld	fa6,56(sp)
    55af:	6970732f          	0x6970732f
    55b3:	2d6e                	fld	fs10,216(sp)
    55b5:	2e30                	fld	fa2,88(a2)
    55b7:	2e35                	addiw	t3,t3,13
    55b9:	2f32                	fld	ft10,264(sp)
    55bb:	2f637273          	csrrci	tp,0x2f6,6
    55bf:	65636e6f          	jal	t3,3bc15 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x2caa5>
    55c3:	722e                	ld	tp,232(sp)
    55c5:	          	0x69000073

00000000000055c8 <anon.d80fe1a93814aa1e9fa97fc113a24de8.2.llvm.13337655799424603394>:
    55c8:	5569                	li	a0,-6
    55ca:	0000                	unimp
    55cc:	0000                	unimp
    55ce:	0000                	unimp
    55d0:	005d                	c.nop	23
    55d2:	0000                	unimp
    55d4:	0000                	unimp
    55d6:	0000                	unimp
    55d8:	0080                	addi	s0,sp,64
    55da:	0000                	unimp
    55dc:	001d                	c.nop	7
	...

00000000000055e0 <anon.d80fe1a93814aa1e9fa97fc113a24de8.3.llvm.13337655799424603394>:
    55e0:	6e69                	lui	t3,0x1a
    55e2:	6574                	ld	a3,200(a0)
    55e4:	6e72                	ld	t3,280(sp)
    55e6:	6c61                	lui	s8,0x18
    55e8:	6520                	ld	s0,72(a0)
    55ea:	7272                	ld	tp,312(sp)
    55ec:	203a726f          	jal	tp,acfee <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x9de7e>
    55f0:	6e65                	lui	t3,0x19
    55f2:	6574                	ld	a3,200(a0)
    55f4:	6572                	ld	a0,280(sp)
    55f6:	2064                	fld	fs1,192(s0)
    55f8:	6e75                	lui	t3,0x1d
    55fa:	6572                	ld	a0,280(sp)
    55fc:	6361                	lui	t1,0x18
    55fe:	6168                	ld	a0,192(a0)
    5600:	6c62                	ld	s8,24(sp)
    5602:	2065                	0x2065
    5604:	65646f63          	bltu	s0,s6,5c62 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.315+0xa>

0000000000005608 <anon.d80fe1a93814aa1e9fa97fc113a24de8.4.llvm.13337655799424603394>:
    5608:	5569                	li	a0,-6
    560a:	0000                	unimp
    560c:	0000                	unimp
    560e:	0000                	unimp
    5610:	005d                	c.nop	23
    5612:	0000                	unimp
    5614:	0000                	unimp
    5616:	0000                	unimp
    5618:	0000007b          	0x7b
    561c:	001f 0000       	0x73610000001f

0000000000005620 <.Lanon.9ecd2340286333da0be21283f7862ea4.0>:
    5620:	7361                	lui	t1,0xffff8
    5622:	74726573          	csrrsi	a0,0x747,4
    5626:	6f69                	lui	t5,0x1a
    5628:	206e                	fld	ft0,216(sp)
    562a:	6166                	ld	sp,88(sp)
    562c:	6c69                	lui	s8,0x1a
    562e:	6465                	lui	s0,0x19
    5630:	203a                	fld	ft0,392(sp)
    5632:	72617473          	csrrci	s0,0x726,2
    5636:	2074                	fld	fa3,192(s0)
    5638:	3d3c                	fld	fa5,120(a0)
    563a:	6520                	ld	s0,72(a0)
    563c:	646e                	ld	s0,216(sp)

000000000000563e <.Lanon.9ecd2340286333da0be21283f7862ea4.1>:
    563e:	6d6f682f          	0x6d6f682f
    5642:	2f65                	addiw	t5,t5,25
    5644:	7568                	ld	a0,232(a0)
    5646:	63637473          	csrrci	s0,0x636,6
    564a:	632e2f63          	0x632e2f63
    564e:	7261                	lui	tp,0xffff8
    5650:	722f6f67          	0x722f6f67
    5654:	6765                	lui	a4,0x19
    5656:	7369                	lui	t1,0xffffa
    5658:	7274                	ld	a3,224(a2)
    565a:	2f79                	addiw	t5,t5,30
    565c:	2f637273          	csrrci	tp,0x2f6,6
    5660:	696d                	lui	s2,0x1b
    5662:	7272                	ld	tp,312(sp)
    5664:	2e73726f          	jal	tp,3d14a <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x2dfda>
    5668:	7375                	lui	t1,0xffffd
    566a:	6374                	ld	a3,192(a4)
    566c:	652e                	ld	a0,200(sp)
    566e:	7564                	ld	s1,232(a0)
    5670:	632e                	ld	t1,200(sp)
    5672:	2d6e                	fld	fs10,216(sp)
    5674:	3136                	fld	ft2,360(sp)
    5676:	6665                	lui	a2,0x19
    5678:	6536                	ld	a0,328(sp)
    567a:	6330                	ld	a2,64(a4)
    567c:	3064                	fld	fs1,224(s0)
    567e:	6636                	ld	a2,328(sp)
    5680:	3962                	fld	fs2,56(sp)
    5682:	3862                	fld	fa6,56(sp)
    5684:	6475622f          	0x6475622f
    5688:	7964                	ld	s1,240(a0)
    568a:	735f 7379 6574      	0x65747379735f
    5690:	5f6d                	li	t5,-5
    5692:	6c61                	lui	s8,0x18
    5694:	6f6c                	ld	a1,216(a4)
    5696:	6f746163          	bltu	s0,s7,5d78 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.344+0x8>
    569a:	2d72                	fld	fs10,280(sp)
    569c:	2e30                	fld	fa2,88(a2)
    569e:	2e36                	fld	ft8,328(sp)
    56a0:	2f30                	fld	fa2,88(a4)
    56a2:	2f637273          	csrrci	tp,0x2f6,6
    56a6:	696c                	ld	a1,208(a0)
    56a8:	2e62                	fld	ft8,24(sp)
    56aa:	7372                	ld	t1,312(sp)
    56ac:	0000                	unimp
	...

00000000000056b0 <.Lanon.9ecd2340286333da0be21283f7862ea4.2>:
    56b0:	563e                	lw	a2,236(sp)
    56b2:	0000                	unimp
    56b4:	0000                	unimp
    56b6:	0000                	unimp
    56b8:	006e                	c.slli	zero,0x1b
    56ba:	0000                	unimp
    56bc:	0000                	unimp
    56be:	0000                	unimp
    56c0:	0050                	addi	a2,sp,4
    56c2:	0000                	unimp
    56c4:	0009                	c.nop	2
	...

00000000000056c8 <.Lanon.9ecd2340286333da0be21283f7862ea4.3>:
    56c8:	563e                	lw	a2,236(sp)
    56ca:	0000                	unimp
    56cc:	0000                	unimp
    56ce:	0000                	unimp
    56d0:	006e                	c.slli	zero,0x1b
    56d2:	0000                	unimp
    56d4:	0000                	unimp
    56d6:	0000                	unimp
    56d8:	005a                	c.slli	zero,0x16
    56da:	0000                	unimp
    56dc:	000d                	c.nop	3
	...

00000000000056e0 <.Lanon.9ecd2340286333da0be21283f7862ea4.5>:
    56e0:	563e                	lw	a2,236(sp)
    56e2:	0000                	unimp
    56e4:	0000                	unimp
    56e6:	0000                	unimp
    56e8:	006e                	c.slli	zero,0x1b
    56ea:	0000                	unimp
    56ec:	0000                	unimp
    56ee:	0000                	unimp
    56f0:	0072                	c.slli	zero,0x1c
    56f2:	0000                	unimp
    56f4:	002a                	c.slli	zero,0xa
	...

00000000000056f8 <.Lanon.9ecd2340286333da0be21283f7862ea4.6>:
    56f8:	563e                	lw	a2,236(sp)
    56fa:	0000                	unimp
    56fc:	0000                	unimp
    56fe:	0000                	unimp
    5700:	006e                	c.slli	zero,0x1b
    5702:	0000                	unimp
    5704:	0000                	unimp
    5706:	0000                	unimp
    5708:	0074                	addi	a3,sp,12
    570a:	0000                	unimp
    570c:	001d                	c.nop	7
	...

0000000000005710 <.Lanon.9ecd2340286333da0be21283f7862ea4.7>:
    5710:	563e                	lw	a2,236(sp)
    5712:	0000                	unimp
    5714:	0000                	unimp
    5716:	0000                	unimp
    5718:	006e                	c.slli	zero,0x1b
    571a:	0000                	unimp
    571c:	0000                	unimp
    571e:	0000                	unimp
    5720:	007e                	c.slli	zero,0x1f
    5722:	0000                	unimp
    5724:	0015                	c.nop	5
	...

0000000000005728 <.Lanon.9ecd2340286333da0be21283f7862ea4.8>:
    5728:	72727563          	bgeu	tp,t2,5e52 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0x38>
    572c:	6e65                	lui	t3,0x19
    572e:	2074                	fld	fa3,192(s0)
    5730:	6c62                	ld	s8,24(sp)
    5732:	206b636f          	jal	t1,bb938 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xac7c8>
    5736:	756f6873          	csrrsi	a6,0x756,30
    573a:	646c                	ld	a1,200(s0)
    573c:	6820                	ld	s0,80(s0)
    573e:	7661                	lui	a2,0xffff8
    5740:	2065                	0x2065
    5742:	7266                	ld	tp,120(sp)
    5744:	6565                	lui	a0,0x19
    5746:	7320                	ld	s0,96(a4)
    5748:	6170                	ld	a2,192(a0)
    574a:	6e206563          	bltu	zero,sp,5e34 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0x1a>
    574e:	          	jal	a4,ed4b0 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xde340>

0000000000005750 <.Lanon.9ecd2340286333da0be21283f7862ea4.9>:
    5750:	563e                	lw	a2,236(sp)
    5752:	0000                	unimp
    5754:	0000                	unimp
    5756:	0000                	unimp
    5758:	006e                	c.slli	zero,0x1b
    575a:	0000                	unimp
    575c:	0000                	unimp
    575e:	0000                	unimp
    5760:	0080                	addi	s0,sp,64
    5762:	0000                	unimp
    5764:	001a                	c.slli	zero,0x6
	...

0000000000005768 <.Lanon.9ecd2340286333da0be21283f7862ea4.10>:
    5768:	563e                	lw	a2,236(sp)
    576a:	0000                	unimp
    576c:	0000                	unimp
    576e:	0000                	unimp
    5770:	006e                	c.slli	zero,0x1b
    5772:	0000                	unimp
    5774:	0000                	unimp
    5776:	0000                	unimp
    5778:	0099                	addi	ra,ra,6
    577a:	0000                	unimp
    577c:	000d                	c.nop	3
	...

0000000000005780 <.Lanon.9ecd2340286333da0be21283f7862ea4.11>:
    5780:	563e                	lw	a2,236(sp)
    5782:	0000                	unimp
    5784:	0000                	unimp
    5786:	0000                	unimp
    5788:	006e                	c.slli	zero,0x1b
    578a:	0000                	unimp
    578c:	0000                	unimp
    578e:	0000                	unimp
    5790:	00ae                	slli	ra,ra,0xb
    5792:	0000                	unimp
    5794:	0015                	c.nop	5
	...

0000000000005798 <.Lanon.ef313fdb91fd0bc7c97c02b65db35f6c.18>:
    5798:	696c                	ld	a1,208(a0)
    579a:	7262                	ld	tp,56(sp)
    579c:	7261                	lui	tp,0xffff8
    579e:	2f79                	addiw	t5,t5,30
    57a0:	6c61                	lui	s8,0x18
    57a2:	6f6c                	ld	a1,216(a4)
    57a4:	72732f63          	0x72732f63
    57a8:	61722f63          	0x61722f63
    57ac:	65765f77          	0x65765f77
    57b0:	73722e63          	0x73722e63

00000000000057b4 <.Lanon.ef313fdb91fd0bc7c97c02b65db35f6c.19>:
    57b4:	61706163          	bltu	zero,s7,5db6 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.346+0x6>
    57b8:	79746963          	bltu	s0,s7,5f4a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.349+0xe>
    57bc:	6f20                	ld	s0,88(a4)
    57be:	6576                	ld	a0,344(sp)
    57c0:	6672                	ld	a2,280(sp)
    57c2:	6f6c                	ld	a1,216(a4)
    57c4:	00000077          	0x77

00000000000057c8 <.Lanon.ef313fdb91fd0bc7c97c02b65db35f6c.20>:
    57c8:	5798                	lw	a4,40(a5)
    57ca:	0000                	unimp
    57cc:	0000                	unimp
    57ce:	0000                	unimp
    57d0:	001c                	0x1c
    57d2:	0000                	unimp
    57d4:	0000                	unimp
    57d6:	0000                	unimp
    57d8:	0218                	addi	a4,sp,256
    57da:	0000                	unimp
    57dc:	0005                	c.nop	1
	...

00000000000057e0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.134>:
    57e0:	2e2e                	fld	ft8,200(sp)
    57e2:	0000                	unimp
    57e4:	0000                	unimp
	...

00000000000057e8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.135>:
    57e8:	57e0                	lw	s0,108(a5)
    57ea:	0000                	unimp
    57ec:	0000                	unimp
    57ee:	0000                	unimp
    57f0:	0002                	c.slli64	zero
    57f2:	0000                	unimp
    57f4:	0000                	unimp
	...

00000000000057f8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.169>:
    57f8:	6c6c6163          	bltu	s8,t1,5eba <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0xa0>
    57fc:	6465                	lui	s0,0x19
    57fe:	6020                	ld	s0,64(s0)
    5800:	6974704f          	fnmadd.s	ft0,fs0,fs7,fa3
    5804:	3a3a6e6f          	jal	t3,ac3a6 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x9d236>
    5808:	6e75                	lui	t3,0x1d
    580a:	70617277          	0x70617277
    580e:	2928                	fld	fa0,80(a0)
    5810:	2060                	fld	fs0,192(s0)
    5812:	61206e6f          	jal	t3,be24 <_ZN12tornado_user10HEAP_SPACE17hcda5f67e49da29e7E+0x4dec>
    5816:	6020                	ld	s0,64(s0)
    5818:	6f4e                	ld	t5,208(sp)
    581a:	656e                	ld	a0,216(sp)
    581c:	2060                	fld	fs0,192(s0)
    581e:	6176                	ld	sp,344(sp)
    5820:	756c                	ld	a1,232(a0)
    5822:	0065                	c.nop	25
    5824:	0000                	unimp
	...

0000000000005828 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.170>:
    5828:	57e0                	lw	s0,108(a5)
	...

0000000000005838 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.171>:
    5838:	203a                	fld	ft0,392(sp)
    583a:	0000                	unimp
    583c:	0000                	unimp
	...

0000000000005840 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.173>:
    5840:	1e86                	slli	t4,t4,0x21
	...
    584e:	0000                	unimp
    5850:	0001                	nop
    5852:	0000                	unimp
    5854:	0000                	unimp
    5856:	0000                	unimp
    5858:	1eec                	addi	a1,sp,892
    585a:	0000                	unimp
    585c:	0000                	unimp
    585e:	0000                	unimp
    5860:	696c                	ld	a1,208(a0)
    5862:	7262                	ld	tp,56(sp)
    5864:	7261                	lui	tp,0xffff8
    5866:	2f79                	addiw	t5,t5,30
    5868:	65726f63          	bltu	tp,s7,5ec6 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0xac>
    586c:	6372732f          	0x6372732f
    5870:	696c732f          	0x696c732f
    5874:	6d2f6563          	bltu	t5,s2,5f3e <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.349+0x2>
    5878:	6d65                	lui	s10,0x19
    587a:	2e726863          	bltu	tp,t2,5b6a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.245+0x2>
    587e:	7372                	ld	t1,312(sp)
    5880:	6e69                	lui	t3,0x1a
    5882:	6564                	ld	s1,200(a0)
    5884:	2078                	fld	fa4,192(s0)
    5886:	2074756f          	jal	a0,4d28c <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x3e11c>
    588a:	6220666f          	jal	a2,beac <_ZN12tornado_user10HEAP_SPACE17hcda5f67e49da29e7E+0x4e74>
    588e:	646e756f          	jal	a0,eced4 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xddd64>
    5892:	74203a73          	csrrc	s4,0x742,zero
    5896:	6568                	ld	a0,200(a0)
    5898:	6c20                	ld	s0,88(s0)
    589a:	6e65                	lui	t3,0x19
    589c:	6920                	ld	s0,80(a0)
    589e:	696c2073          	csrs	0x696,s8
    58a2:	7262                	ld	tp,56(sp)
    58a4:	7261                	lui	tp,0xffff8
    58a6:	2f79                	addiw	t5,t5,30
    58a8:	65726f63          	bltu	tp,s7,5f06 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0xec>
    58ac:	6372732f          	0x6372732f
    58b0:	746d662f          	0x746d662f
    58b4:	6975622f          	0x6975622f
    58b8:	646c                	ld	a1,200(s0)
    58ba:	7265                	lui	tp,0xffff9
    58bc:	73722e73          	csrrs	t3,0x737,tp

00000000000058c0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.181>:
    58c0:	6220                	ld	s0,64(a2)
    58c2:	7475                	lui	s0,0xffffd
    58c4:	7420                	ld	s0,104(s0)
    58c6:	6568                	ld	a0,200(a0)
    58c8:	6920                	ld	s0,80(a0)
    58ca:	646e                	ld	s0,216(sp)
    58cc:	7865                	lui	a6,0xffff9
    58ce:	6920                	ld	s0,80(a0)
    58d0:	00002073          	csrr	zero,ustatus
    58d4:	0000                	unimp
	...

00000000000058d8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.182>:
    58d8:	5880                	lw	s0,48(s1)
    58da:	0000                	unimp
    58dc:	0000                	unimp
    58de:	0000                	unimp
    58e0:	0020                	addi	s0,sp,8
    58e2:	0000                	unimp
    58e4:	0000                	unimp
    58e6:	0000                	unimp
    58e8:	58c0                	lw	s0,52(s1)
    58ea:	0000                	unimp
    58ec:	0000                	unimp
    58ee:	0000                	unimp
    58f0:	0012                	c.slli	zero,0x4
    58f2:	0000                	unimp
    58f4:	0000                	unimp
	...

00000000000058f8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.184>:
    58f8:	3d21                	addiw	s10,s10,-24

00000000000058fa <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.185>:
    58fa:	3d3d                	addiw	s10,s10,-17

00000000000058fc <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.186>:
    58fc:	7361                	lui	t1,0xffff8
    58fe:	74726573          	csrrsi	a0,0x747,4
    5902:	6f69                	lui	t5,0x1a
    5904:	206e                	fld	ft0,216(sp)
    5906:	6166                	ld	sp,88(sp)
    5908:	6c69                	lui	s8,0x1a
    590a:	6465                	lui	s0,0x19
    590c:	203a                	fld	ft0,392(sp)
    590e:	2860                	fld	fs0,208(s0)
    5910:	656c                	ld	a1,200(a0)
    5912:	7466                	ld	s0,120(sp)
    5914:	                	fld	fs0,64(s0)

0000000000005915 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.187>:
    5915:	7220                	ld	s0,96(a2)
    5917:	6769                	lui	a4,0x1a
    5919:	7468                	ld	a0,232(s0)
    591b:	6029                	c.lui	zero,0xa
    591d:	200a                	fld	ft0,128(sp)
    591f:	6c20                	ld	s0,88(s0)
    5921:	6665                	lui	a2,0x19
    5923:	3a74                	fld	fa3,240(a2)
    5925:	6020                	ld	s0,64(s0)

0000000000005927 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.188>:
    5927:	2c60                	fld	fs0,216(s0)
    5929:	200a                	fld	ft0,128(sp)
    592b:	6972                	ld	s2,280(sp)
    592d:	3a746867          	0x3a746867
    5931:	6020                	ld	s0,64(s0)

0000000000005933 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.189>:
    5933:	0060                	addi	s0,sp,12
    5935:	0000                	unimp
	...

0000000000005938 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.190>:
    5938:	58fc                	lw	a5,116(s1)
    593a:	0000                	unimp
    593c:	0000                	unimp
    593e:	0000                	unimp
    5940:	0019                	c.nop	6
    5942:	0000                	unimp
    5944:	0000                	unimp
    5946:	0000                	unimp
    5948:	5915                	li	s2,-27
    594a:	0000                	unimp
    594c:	0000                	unimp
    594e:	0000                	unimp
    5950:	0012                	c.slli	zero,0x4
    5952:	0000                	unimp
    5954:	0000                	unimp
    5956:	0000                	unimp
    5958:	00005927          	0x5927
    595c:	0000                	unimp
    595e:	0000                	unimp
    5960:	000c                	0xc
    5962:	0000                	unimp
    5964:	0000                	unimp
    5966:	0000                	unimp
    5968:	5838                	lw	a4,112(s0)
    596a:	0000                	unimp
    596c:	0000                	unimp
    596e:	0000                	unimp
    5970:	0002                	c.slli64	zero
    5972:	0000                	unimp
    5974:	0000                	unimp
    5976:	0000                	unimp
    5978:	00005933          	srl	s2,zero,zero
    597c:	0000                	unimp
    597e:	0000                	unimp
    5980:	0001                	nop
    5982:	0000                	unimp
    5984:	0000                	unimp
	...

0000000000005988 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.191>:
    5988:	58fc                	lw	a5,116(s1)
    598a:	0000                	unimp
    598c:	0000                	unimp
    598e:	0000                	unimp
    5990:	0019                	c.nop	6
    5992:	0000                	unimp
    5994:	0000                	unimp
    5996:	0000                	unimp
    5998:	5915                	li	s2,-27
    599a:	0000                	unimp
    599c:	0000                	unimp
    599e:	0000                	unimp
    59a0:	0012                	c.slli	zero,0x4
    59a2:	0000                	unimp
    59a4:	0000                	unimp
    59a6:	0000                	unimp
    59a8:	00005927          	0x5927
    59ac:	0000                	unimp
    59ae:	0000                	unimp
    59b0:	000c                	0xc
    59b2:	0000                	unimp
    59b4:	0000                	unimp
    59b6:	0000                	unimp
    59b8:	00005933          	srl	s2,zero,zero
    59bc:	0000                	unimp
    59be:	0000                	unimp
    59c0:	0001                	nop
    59c2:	0000                	unimp
    59c4:	0000                	unimp
	...

00000000000059c8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.193>:
    59c8:	1e86                	slli	t4,t4,0x21
    59ca:	0000                	unimp
    59cc:	0000                	unimp
    59ce:	0000                	unimp
    59d0:	0018                	0x18
    59d2:	0000                	unimp
    59d4:	0000                	unimp
    59d6:	0000                	unimp
    59d8:	0008                	0x8
    59da:	0000                	unimp
    59dc:	0000                	unimp
    59de:	0000                	unimp
    59e0:	20a8                	fld	fa0,64(s1)
    59e2:	0000                	unimp
    59e4:	0000                	unimp
    59e6:	0000                	unimp
    59e8:	2472                	fld	fs0,280(sp)
    59ea:	0000                	unimp
    59ec:	0000                	unimp
    59ee:	0000                	unimp
    59f0:	2552                	fld	fa0,272(sp)
    59f2:	0000                	unimp
    59f4:	0000                	unimp
	...

00000000000059f8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.195>:
    59f8:	58a0                	lw	s0,112(s1)
    59fa:	0000                	unimp
    59fc:	0000                	unimp
    59fe:	0000                	unimp
    5a00:	0020                	addi	s0,sp,8
    5a02:	0000                	unimp
    5a04:	0000                	unimp
    5a06:	0000                	unimp
    5a08:	0032                	c.slli	zero,0xc
    5a0a:	0000                	unimp
    5a0c:	0021                	c.nop	8
	...

0000000000005a10 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.196>:
    5a10:	58a0                	lw	s0,112(s1)
    5a12:	0000                	unimp
    5a14:	0000                	unimp
    5a16:	0000                	unimp
    5a18:	0020                	addi	s0,sp,8
    5a1a:	0000                	unimp
    5a1c:	0000                	unimp
    5a1e:	0000                	unimp
    5a20:	00000033          	add	zero,zero,zero
    5a24:	0012                	c.slli	zero,0x4
	...

0000000000005a28 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.198>:
    5a28:	0a2c                	addi	a1,sp,280

0000000000005a2a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.199>:
    5a2a:	202c                	fld	fa1,64(s0)

0000000000005a2c <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.206>:
    5a2c:	0a28                	addi	a0,sp,280

0000000000005a2e <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.207>:
    5a2e:	                	fld	fa0,88(s0)

0000000000005a2f <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.208>:
    5a2f:	                	fld	fa1,80(a0)

0000000000005a30 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.209>:
    5a30:	                	addi	s4,s4,10

0000000000005a31 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.210>:
    5a31:	                	lw	s6,160(sp)

0000000000005a32 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.212>:
    5a32:	          	0x696c5d5b

0000000000005a33 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.216>:
    5a33:	                	lui	s8,0x17

0000000000005a34 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.223>:
    5a34:	696c                	ld	a1,208(a0)
    5a36:	7262                	ld	tp,56(sp)
    5a38:	7261                	lui	tp,0xffff8
    5a3a:	2f79                	addiw	t5,t5,30
    5a3c:	65726f63          	bltu	tp,s7,609a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.350+0x29>
    5a40:	6372732f          	0x6372732f
    5a44:	746d662f          	0x746d662f
    5a48:	6d756e2f          	0x6d756e2f
    5a4c:	722e                	ld	tp,232(sp)
    5a4e:	          	0x5a340073

0000000000005a50 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.224>:
    5a50:	5a34                	lw	a3,112(a2)
    5a52:	0000                	unimp
    5a54:	0000                	unimp
    5a56:	0000                	unimp
    5a58:	0000001b          	sext.w	zero,zero
    5a5c:	0000                	unimp
    5a5e:	0000                	unimp
    5a60:	0065                	c.nop	25
    5a62:	0000                	unimp
    5a64:	0014                	0x14
	...

0000000000005a68 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.226>:
    5a68:	7830                	ld	a2,112(s0)

0000000000005a6a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.228>:
    5a6a:	3030                	fld	fa2,96(s0)
    5a6c:	3130                	fld	fa2,96(a0)
    5a6e:	3230                	fld	fa2,96(a2)
    5a70:	3330                	fld	fa2,96(a4)
    5a72:	3430                	fld	fa2,104(s0)
    5a74:	3530                	fld	fa2,104(a0)
    5a76:	3630                	fld	fa2,104(a2)
    5a78:	3730                	fld	fa2,104(a4)
    5a7a:	3830                	fld	fa2,112(s0)
    5a7c:	3930                	fld	fa2,112(a0)
    5a7e:	3031                	0x3031
    5a80:	3131                	addiw	sp,sp,-20
    5a82:	3231                	addiw	tp,tp,-20
    5a84:	3331                	addiw	t1,t1,-20
    5a86:	3431                	addiw	s0,s0,-20
    5a88:	3531                	addiw	a0,a0,-20
    5a8a:	3631                	addiw	a2,a2,-20
    5a8c:	3731                	addiw	a4,a4,-20
    5a8e:	3831                	addiw	a6,a6,-20
    5a90:	3931                	addiw	s2,s2,-20
    5a92:	3032                	fld	ft0,296(sp)
    5a94:	3132                	fld	ft2,296(sp)
    5a96:	3232                	fld	ft4,296(sp)
    5a98:	3332                	fld	ft6,296(sp)
    5a9a:	3432                	fld	fs0,296(sp)
    5a9c:	3532                	fld	fa0,296(sp)
    5a9e:	3632                	fld	fa2,296(sp)
    5aa0:	3732                	fld	fa4,296(sp)
    5aa2:	3832                	fld	fa6,296(sp)
    5aa4:	3932                	fld	fs2,296(sp)
    5aa6:	31333033          	0x31333033
    5aaa:	33333233          	0x33333233
    5aae:	35333433          	0x35333433
    5ab2:	37333633          	0x37333633
    5ab6:	39333833          	0x39333833
    5aba:	3034                	fld	fa3,96(s0)
    5abc:	3134                	fld	fa3,96(a0)
    5abe:	3234                	fld	fa3,96(a2)
    5ac0:	3334                	fld	fa3,96(a4)
    5ac2:	3434                	fld	fa3,104(s0)
    5ac4:	3534                	fld	fa3,104(a0)
    5ac6:	3634                	fld	fa3,104(a2)
    5ac8:	3734                	fld	fa3,104(a4)
    5aca:	3834                	fld	fa3,112(s0)
    5acc:	3934                	fld	fa3,112(a0)
    5ace:	3035                	0x3035
    5ad0:	3135                	addiw	sp,sp,-19
    5ad2:	3235                	addiw	tp,tp,-19
    5ad4:	3335                	addiw	t1,t1,-19
    5ad6:	3435                	addiw	s0,s0,-19
    5ad8:	3535                	addiw	a0,a0,-19
    5ada:	3635                	addiw	a2,a2,-19
    5adc:	3735                	addiw	a4,a4,-19
    5ade:	3835                	addiw	a6,a6,-19
    5ae0:	3935                	addiw	s2,s2,-19
    5ae2:	3036                	fld	ft0,360(sp)
    5ae4:	3136                	fld	ft2,360(sp)
    5ae6:	3236                	fld	ft4,360(sp)
    5ae8:	3336                	fld	ft6,360(sp)
    5aea:	3436                	fld	fs0,360(sp)
    5aec:	3536                	fld	fa0,360(sp)
    5aee:	3636                	fld	fa2,360(sp)
    5af0:	3736                	fld	fa4,360(sp)
    5af2:	3836                	fld	fa6,360(sp)
    5af4:	3936                	fld	fs2,360(sp)
    5af6:	31373037          	lui	zero,0x31373
    5afa:	33373237          	lui	tp,0x33373
    5afe:	35373437          	lui	s0,0x35373
    5b02:	37373637          	lui	a2,0x37373
    5b06:	39373837          	lui	a6,0x39373
    5b0a:	3038                	fld	fa4,96(s0)
    5b0c:	3138                	fld	fa4,96(a0)
    5b0e:	3238                	fld	fa4,96(a2)
    5b10:	3338                	fld	fa4,96(a4)
    5b12:	3438                	fld	fa4,104(s0)
    5b14:	3538                	fld	fa4,104(a0)
    5b16:	3638                	fld	fa4,104(a2)
    5b18:	3738                	fld	fa4,104(a4)
    5b1a:	3838                	fld	fa4,112(s0)
    5b1c:	3938                	fld	fa4,112(a0)
    5b1e:	3039                	0x3039
    5b20:	3139                	addiw	sp,sp,-18
    5b22:	3239                	addiw	tp,tp,-18
    5b24:	3339                	addiw	t1,t1,-18
    5b26:	3439                	addiw	s0,s0,-18
    5b28:	3539                	addiw	a0,a0,-18
    5b2a:	3639                	addiw	a2,a2,-18
    5b2c:	3739                	addiw	a4,a4,-18
    5b2e:	3839                	addiw	a6,a6,-18
    5b30:	3939                	addiw	s2,s2,-18
    5b32:	0000                	unimp
    5b34:	0000                	unimp
	...

0000000000005b38 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.231>:
    5b38:	1e86                	slli	t4,t4,0x21
    5b3a:	0000                	unimp
    5b3c:	0000                	unimp
    5b3e:	0000                	unimp
    5b40:	0008                	0x8
    5b42:	0000                	unimp
    5b44:	0000                	unimp
    5b46:	0000                	unimp
    5b48:	0008                	0x8
    5b4a:	0000                	unimp
    5b4c:	0000                	unimp
    5b4e:	0000                	unimp
    5b50:	258a                	fld	fa1,128(sp)
    5b52:	0000                	unimp
    5b54:	0000                	unimp
    5b56:	0000                	unimp
    5b58:	2594                	fld	fa3,8(a1)
    5b5a:	0000                	unimp
    5b5c:	0000                	unimp
    5b5e:	0000                	unimp
    5b60:	2676                	fld	fa2,344(sp)
    5b62:	0000                	unimp
    5b64:	0000                	unimp
	...

0000000000005b68 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.245>:
    5b68:	5860                	lw	s0,116(s0)
    5b6a:	0000                	unimp
    5b6c:	0000                	unimp
    5b6e:	0000                	unimp
    5b70:	0020                	addi	s0,sp,8
    5b72:	0000                	unimp
    5b74:	0000                	unimp
    5b76:	0000                	unimp
    5b78:	005a                	c.slli	zero,0x16
    5b7a:	0000                	unimp
    5b7c:	0005                	c.nop	1
	...

0000000000005b80 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.248>:
    5b80:	6172                	ld	sp,280(sp)
    5b82:	676e                	ld	a4,216(sp)
    5b84:	2065                	0x2065
    5b86:	72617473          	csrrci	s0,0x726,2
    5b8a:	2074                	fld	fa3,192(s0)
    5b8c:	6e69                	lui	t3,0x1a
    5b8e:	6564                	ld	s1,200(a0)
    5b90:	2078                	fld	fa4,192(s0)

0000000000005b92 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.249>:
    5b92:	6f20                	ld	s0,88(a4)
    5b94:	7475                	lui	s0,0xffffd
    5b96:	6f20                	ld	s0,88(a4)
    5b98:	2066                	fld	ft0,88(sp)
    5b9a:	6172                	ld	sp,280(sp)
    5b9c:	676e                	ld	a4,216(sp)
    5b9e:	2065                	0x2065
    5ba0:	6f66                	ld	t5,88(sp)
    5ba2:	2072                	fld	ft0,280(sp)
    5ba4:	63696c73          	csrrsi	s8,0x636,18
    5ba8:	2065                	0x2065
    5baa:	6c20666f          	jal	a2,c26c <_ZN12tornado_user10HEAP_SPACE17hcda5f67e49da29e7E+0x5234>
    5bae:	6e65                	lui	t3,0x19
    5bb0:	20687467          	0x20687467
    5bb4:	0000                	unimp
	...

0000000000005bb8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.250>:
    5bb8:	5b80                	lw	s0,48(a5)
    5bba:	0000                	unimp
    5bbc:	0000                	unimp
    5bbe:	0000                	unimp
    5bc0:	0012                	c.slli	zero,0x4
    5bc2:	0000                	unimp
    5bc4:	0000                	unimp
    5bc6:	0000                	unimp
    5bc8:	5b92                	lw	s7,36(sp)
    5bca:	0000                	unimp
    5bcc:	0000                	unimp
    5bce:	0000                	unimp
    5bd0:	0022                	c.slli	zero,0x8
    5bd2:	0000                	unimp
    5bd4:	0000                	unimp
	...

0000000000005bd8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.252>:
    5bd8:	5060                	lw	s0,100(s0)
    5bda:	0000                	unimp
    5bdc:	0000                	unimp
    5bde:	0000                	unimp
    5be0:	0010                	0x10
    5be2:	0000                	unimp
    5be4:	0000                	unimp
    5be6:	0000                	unimp
    5be8:	5b92                	lw	s7,36(sp)
    5bea:	0000                	unimp
    5bec:	0000                	unimp
    5bee:	0000                	unimp
    5bf0:	0022                	c.slli	zero,0x8
    5bf2:	0000                	unimp
    5bf4:	0000                	unimp
	...

0000000000005bf8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.253>:
    5bf8:	63696c73          	csrrsi	s8,0x636,18
    5bfc:	2065                	0x2065
    5bfe:	6e69                	lui	t3,0x1a
    5c00:	6564                	ld	s1,200(a0)
    5c02:	2078                	fld	fa4,192(s0)
    5c04:	72617473          	csrrci	s0,0x726,2
    5c08:	7374                	ld	a3,224(a4)
    5c0a:	6120                	ld	s0,64(a0)
    5c0c:	2074                	fld	fa3,192(s0)

0000000000005c0e <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.254>:
    5c0e:	6220                	ld	s0,64(a2)
    5c10:	7475                	lui	s0,0xffffd
    5c12:	6520                	ld	s0,72(a0)
    5c14:	646e                	ld	s0,216(sp)
    5c16:	74612073          	csrs	0x746,sp
    5c1a:	0020                	addi	s0,sp,8
    5c1c:	0000                	unimp
	...

0000000000005c20 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.255>:
    5c20:	5bf8                	lw	a4,116(a5)
    5c22:	0000                	unimp
    5c24:	0000                	unimp
    5c26:	0000                	unimp
    5c28:	0016                	c.slli	zero,0x5
    5c2a:	0000                	unimp
    5c2c:	0000                	unimp
    5c2e:	0000                	unimp
    5c30:	5c0e                	lw	s8,224(sp)
    5c32:	0000                	unimp
    5c34:	0000                	unimp
    5c36:	0000                	unimp
    5c38:	000d                	c.nop	3
    5c3a:	0000                	unimp
    5c3c:	0000                	unimp
    5c3e:	0000                	unimp
    5c40:	2820                	fld	fs0,80(s0)
    5c42:	7962                	ld	s2,56(sp)
    5c44:	6574                	ld	a3,200(a0)
    5c46:	          	csrs	0x2e5,s6

0000000000005c48 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.313>:
    5c48:	2e2e2e5b          	0x2e2e2e5b
    5c4c:	                	lui	tp,0x17

0000000000005c4d <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.314>:
    5c4d:	7962                	ld	s2,56(sp)
    5c4f:	6574                	ld	a3,200(a0)
    5c51:	6920                	ld	s0,80(a0)
    5c53:	646e                	ld	s0,216(sp)
    5c55:	7865                	lui	a6,0xffff9
    5c57:	                	fld	fs0,64(s0)

0000000000005c58 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.315>:
    5c58:	6920                	ld	s0,80(a0)
    5c5a:	756f2073          	csrs	0x756,t5
    5c5e:	2074                	fld	fa3,192(s0)
    5c60:	6220666f          	jal	a2,c282 <_ZN12tornado_user10HEAP_SPACE17hcda5f67e49da29e7E+0x524a>
    5c64:	646e756f          	jal	a0,ed2aa <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xde13a>
    5c68:	666f2073          	csrs	0x666,t5
    5c6c:	6020                	ld	s0,64(s0)
	...

0000000000005c70 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.316>:
    5c70:	5c4d                	li	s8,-13
    5c72:	0000                	unimp
    5c74:	0000                	unimp
    5c76:	0000                	unimp
    5c78:	0000000b          	0xb
    5c7c:	0000                	unimp
    5c7e:	0000                	unimp
    5c80:	5c58                	lw	a4,60(s0)
    5c82:	0000                	unimp
    5c84:	0000                	unimp
    5c86:	0000                	unimp
    5c88:	0016                	c.slli	zero,0x5
    5c8a:	0000                	unimp
    5c8c:	0000                	unimp
    5c8e:	0000                	unimp
    5c90:	00005933          	srl	s2,zero,zero
    5c94:	0000                	unimp
    5c96:	0000                	unimp
    5c98:	0001                	nop
    5c9a:	0000                	unimp
    5c9c:	0000                	unimp
	...

0000000000005ca0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.317>:
    5ca0:	6562                	ld	a0,24(sp)
    5ca2:	206e6967          	0x206e6967
    5ca6:	3d3c                	fld	fa5,120(a0)
    5ca8:	6520                	ld	s0,72(a0)
    5caa:	646e                	ld	s0,216(sp)
    5cac:	2820                	fld	fs0,80(s0)
	...

0000000000005cb0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.320>:
    5cb0:	5ca0                	lw	s0,120(s1)
    5cb2:	0000                	unimp
    5cb4:	0000                	unimp
    5cb6:	0000                	unimp
    5cb8:	000e                	c.slli	zero,0x3
    5cba:	0000                	unimp
    5cbc:	0000                	unimp
    5cbe:	0000                	unimp
    5cc0:	51f0                	lw	a2,100(a1)
    5cc2:	0000                	unimp
    5cc4:	0000                	unimp
    5cc6:	0000                	unimp
    5cc8:	0004                	0x4
    5cca:	0000                	unimp
    5ccc:	0000                	unimp
    5cce:	0000                	unimp
    5cd0:	5040                	lw	s0,36(s0)
    5cd2:	0000                	unimp
    5cd4:	0000                	unimp
    5cd6:	0000                	unimp
    5cd8:	0010                	0x10
    5cda:	0000                	unimp
    5cdc:	0000                	unimp
    5cde:	0000                	unimp
    5ce0:	00005933          	srl	s2,zero,zero
    5ce4:	0000                	unimp
    5ce6:	0000                	unimp
    5ce8:	0001                	nop
    5cea:	0000                	unimp
    5cec:	0000                	unimp
	...

0000000000005cf0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.321>:
    5cf0:	6920                	ld	s0,80(a0)
    5cf2:	6f6e2073          	csrs	0x6f6,t3
    5cf6:	2074                	fld	fa3,192(s0)
    5cf8:	2061                	0x2061
    5cfa:	72616863          	bltu	sp,t1,642a <_ZN4core7unicode12unicode_data15grapheme_extend7OFFSETS17h21859fdd3179f732E+0x2e>
    5cfe:	6220                	ld	s0,64(a2)
    5d00:	646e756f          	jal	a0,ed346 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xde1d6>
    5d04:	7261                	lui	tp,0xffff8
    5d06:	3b79                	addiw	s6,s6,-2
    5d08:	6920                	ld	s0,80(a0)
    5d0a:	2074                	fld	fa3,192(s0)
    5d0c:	7369                	lui	t1,0xffffa
    5d0e:	6920                	ld	s0,80(a0)
    5d10:	736e                	ld	t1,248(sp)
    5d12:	6469                	lui	s0,0x1a
    5d14:	2065                	0x2065

0000000000005d16 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.323>:
    5d16:	2029                	0x2029
    5d18:	6020666f          	jal	a2,c31a <_ZN12tornado_user10HEAP_SPACE17hcda5f67e49da29e7E+0x52e2>
    5d1c:	0000                	unimp
	...

0000000000005d20 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.324>:
    5d20:	5c4d                	li	s8,-13
    5d22:	0000                	unimp
    5d24:	0000                	unimp
    5d26:	0000                	unimp
    5d28:	0000000b          	0xb
    5d2c:	0000                	unimp
    5d2e:	0000                	unimp
    5d30:	5cf0                	lw	a2,124(s1)
    5d32:	0000                	unimp
    5d34:	0000                	unimp
    5d36:	0000                	unimp
    5d38:	0026                	c.slli	zero,0x9
    5d3a:	0000                	unimp
    5d3c:	0000                	unimp
    5d3e:	0000                	unimp
    5d40:	5c40                	lw	s0,60(s0)
    5d42:	0000                	unimp
    5d44:	0000                	unimp
    5d46:	0000                	unimp
    5d48:	0008                	0x8
    5d4a:	0000                	unimp
    5d4c:	0000                	unimp
    5d4e:	0000                	unimp
    5d50:	5d16                	lw	s10,100(sp)
    5d52:	0000                	unimp
    5d54:	0000                	unimp
    5d56:	0000                	unimp
    5d58:	0006                	c.slli	zero,0x1
    5d5a:	0000                	unimp
    5d5c:	0000                	unimp
    5d5e:	0000                	unimp
    5d60:	00005933          	srl	s2,zero,zero
    5d64:	0000                	unimp
    5d66:	0000                	unimp
    5d68:	0001                	nop
    5d6a:	0000                	unimp
    5d6c:	0000                	unimp
	...

0000000000005d70 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.344>:
    5d70:	696c                	ld	a1,208(a0)
    5d72:	7262                	ld	tp,56(sp)
    5d74:	7261                	lui	tp,0xffff8
    5d76:	2f79                	addiw	t5,t5,30
    5d78:	65726f63          	bltu	tp,s7,63d6 <_ZN4core7unicode12unicode_data15grapheme_extend17SHORT_OFFSET_RUNS17hff6f6dc47b6092b7E+0x56>
    5d7c:	6372732f          	0x6372732f
    5d80:	696e752f          	0x696e752f
    5d84:	65646f63          	bltu	s0,s6,63e2 <_ZN4core7unicode12unicode_data15grapheme_extend17SHORT_OFFSET_RUNS17hff6f6dc47b6092b7E+0x62>
    5d88:	6972702f          	0x6972702f
    5d8c:	746e                	ld	s0,248(sp)
    5d8e:	6261                	lui	tp,0x18
    5d90:	656c                	ld	a1,200(a0)
    5d92:	722e                	ld	tp,232(sp)
    5d94:	00000073          	ecall

0000000000005d98 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.345>:
    5d98:	5d70                	lw	a2,124(a0)
    5d9a:	0000                	unimp
    5d9c:	0000                	unimp
    5d9e:	0000                	unimp
    5da0:	0025                	c.nop	9
    5da2:	0000                	unimp
    5da4:	0000                	unimp
    5da6:	0000                	unimp
    5da8:	000a                	c.slli	zero,0x2
    5daa:	0000                	unimp
    5dac:	001c                	0x1c
	...

0000000000005db0 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.346>:
    5db0:	5d70                	lw	a2,124(a0)
    5db2:	0000                	unimp
    5db4:	0000                	unimp
    5db6:	0000                	unimp
    5db8:	0025                	c.nop	9
    5dba:	0000                	unimp
    5dbc:	0000                	unimp
    5dbe:	0000                	unimp
    5dc0:	001a                	c.slli	zero,0x6
    5dc2:	0000                	unimp
    5dc4:	0036                	c.slli	zero,0xd
	...

0000000000005dc8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.347>:
    5dc8:	0100                	addi	s0,sp,128
    5dca:	06050503          	lb	a0,96(a0) # 19060 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x9ef0>
    5dce:	0306                	slli	t1,t1,0x1
    5dd0:	08080607          	0x8080607
    5dd4:	1109                	addi	sp,sp,-30
    5dd6:	1c0a                	slli	s8,s8,0x22
    5dd8:	140c190b          	0x140c190b
    5ddc:	100d                	c.nop	-29
    5dde:	0d0e                	slli	s10,s10,0x3
    5de0:	0310040f          	0x310040f
    5de4:	1212                	slli	tp,tp,0x24
    5de6:	01160913          	addi	s2,a2,17 # 37373011 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x37363ea1>
    5dea:	02180517          	auipc	a0,0x2180
    5dee:	0319                	addi	t1,t1,6
    5df0:	071a                	slli	a4,a4,0x6
    5df2:	021c                	addi	a5,sp,256
    5df4:	011d                	addi	sp,sp,7
    5df6:	161f 0320 032b      	0x32b0320161f
    5dfc:	022c                	addi	a1,sp,264
    5dfe:	0b2d                	addi	s6,s6,11
    5e00:	012e                	slli	sp,sp,0xb
    5e02:	0330                	addi	a2,sp,392
    5e04:	0231                	addi	tp,tp,12
    5e06:	0132                	slli	sp,sp,0xc
    5e08:	02a902a7          	0x2a902a7
    5e0c:	04aa                	slli	s1,s1,0xa
    5e0e:	02fa08ab          	0x2fa08ab
    5e12:	04fd05fb          	0x4fd05fb
    5e16:	03fe                	slli	t2,t2,0x1f
    5e18:	09ff                	0x9ff

0000000000005e1a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348>:
    5e1a:	78ad                	lui	a7,0xfffeb
    5e1c:	8b79                	andi	a4,a4,30
    5e1e:	a28d                	j	5f80 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.349+0x44>
    5e20:	5730                	lw	a2,104(a4)
    5e22:	8b58                	0x8b58
    5e24:	908c                	0x908c
    5e26:	1d1c                	addi	a5,sp,688
    5e28:	0edd                	addi	t4,t4,23
    5e2a:	fb4c4b0f          	0xfb4c4b0f
    5e2e:	2efc                	fld	fa5,216(a3)
    5e30:	5d5c3f2f          	0x5d5c3f2f
    5e34:	b55f 84e2 8e8d      	0x8e8d84e2b55f
    5e3a:	9291                	srli	a3,a3,0x24
    5e3c:	b1a9                	j	5a86 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.228+0x1c>
    5e3e:	bbba                	fsd	fa4,496(sp)
    5e40:	c6c5                	beqz	a3,5ee8 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0xce>
    5e42:	cac9                	beqz	a3,5ed4 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0xba>
    5e44:	e4de                	sd	s7,72(sp)
    5e46:	ffe5                	bnez	a5,5e3e <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0x24>
    5e48:	0400                	addi	s0,sp,512
    5e4a:	1211                	addi	tp,tp,-28
    5e4c:	3129                	addiw	sp,sp,-22
    5e4e:	3734                	fld	fa3,104(a4)
    5e50:	3b3a                	fld	fs6,424(sp)
    5e52:	493d                	li	s2,15
    5e54:	5d4a                	lw	s10,176(sp)
    5e56:	8e84                	0x8e84
    5e58:	a992                	fsd	ft4,208(sp)
    5e5a:	b4b1                	j	58a6 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.173+0x66>
    5e5c:	bbba                	fsd	fa4,496(sp)
    5e5e:	cac6                	sw	a7,84(sp)
    5e60:	cfce                	sw	s3,220(sp)
    5e62:	e5e4                	sd	s1,200(a1)
    5e64:	0400                	addi	s0,sp,512
    5e66:	0e0d                	addi	t3,t3,3
    5e68:	1211                	addi	tp,tp,-28
    5e6a:	3129                	addiw	sp,sp,-22
    5e6c:	3a34                	fld	fa3,112(a2)
    5e6e:	4946453b          	0x4946453b
    5e72:	5e4a                	lw	t3,176(sp)
    5e74:	6564                	ld	s1,200(a0)
    5e76:	9184                	0x9184
    5e78:	cec99d9b          	0xcec99d9b
    5e7c:	29110dcf          	fnmadd.s	fs11,ft2,fa7,ft5,rne
    5e80:	4945                	li	s2,17
    5e82:	8d656457          	0x8d656457
    5e86:	a991                	j	62da <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.352+0x16e>
    5e88:	bab4                	fsd	fa3,112(a3)
    5e8a:	dfc9c5bb          	0xdfc9c5bb
    5e8e:	e5e4                	sd	s1,200(a1)
    5e90:	0df0                	addi	a2,sp,732
    5e92:	4511                	li	a0,4
    5e94:	6449                	lui	s0,0x12
    5e96:	8065                	srli	s0,s0,0x19
    5e98:	b284                	fsd	fs1,32(a3)
    5e9a:	bebc                	fsd	fa5,120(a3)
    5e9c:	f0d7d5bf 8b8583f1 	0x8b8583f1f0d7d5bf
    5ea4:	a6a4                	fsd	fs1,72(a3)
    5ea6:	bfbe                	fsd	fa5,504(sp)
    5ea8:	c7c5                	beqz	a5,5f50 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.349+0x14>
    5eaa:	cfce                	sw	s3,220(sp)
    5eac:	dbda                	sw	s6,244(sp)
    5eae:	9848                	0x9848
    5eb0:	cdbd                	beqz	a1,5f2e <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0x114>
    5eb2:	cec6                	sw	a7,92(sp)
    5eb4:	4f4e49cf          	fnmadd.q	fs3,ft8,fs4,fs1,rmm
    5eb8:	5f5e5957          	0x5f5e5957
    5ebc:	8e89                	sub	a3,a3,a0
    5ebe:	b7b6b18f          	0xb7b6b18f
    5ec2:	c7c6c1bf 171611d7 	0x171611d7c7c6c1bf
    5eca:	f7f65c5b          	0xf7f65c5b
    5ece:	fffe                	sd	t6,504(sp)
    5ed0:	0d80                	addi	s0,sp,720
    5ed2:	716d                	addi	sp,sp,-272
    5ed4:	dfde                	sw	s7,252(sp)
    5ed6:	0f0e                	slli	t5,t5,0x3
    5ed8:	6e1f 1c6f 5f1d      	0x5f1d1c6f6e1f
    5ede:	7e7d                	lui	t3,0xfffff
    5ee0:	afae                	fsd	fa1,472(sp)
    5ee2:	16fabcbb          	0x16fabcbb
    5ee6:	461f1e17          	auipc	t3,0x461f1
    5eea:	584f4e47          	fmsub.s	ft8,ft10,ft4,fa1,rmm
    5eee:	5c5a                	lw	s8,180(sp)
    5ef0:	7e5e                	ld	t3,496(sp)
    5ef2:	b57f                	0xb57f
    5ef4:	d4c5                	beqz	s1,5e9c <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0x82>
    5ef6:	dcd5                	beqz	s1,5eb2 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0x98>
    5ef8:	f1f0                	sd	a2,224(a1)
    5efa:	72f5                	lui	t0,0xffffd
    5efc:	75748f73          	0x75748f73
    5f00:	2f96                	fld	ft11,320(sp)
    5f02:	265f 2f2e afa7      	0xafa72f2e265f
    5f08:	cfc7bfb7          	lui	t6,0xcfc7b
    5f0c:	409adfd7          	0x409adfd7
    5f10:	8f309897          	auipc	a7,0x8f309
    5f14:	c01f cec1 4eff      	0x4effcec1c01f
    5f1a:	075b5a4f          	fnmadd.q	fs4,fs6,fs5,ft0,unknown
    5f1e:	0f08                	addi	a0,sp,912
    5f20:	2710                	fld	fa2,8(a4)
    5f22:	6eefee2f          	0x6eefee2f
    5f26:	3f3d376f          	jal	a4,d9b18 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xca9a8>
    5f2a:	4542                	lw	a0,16(sp)
    5f2c:	9190                	0x9190
    5f2e:	fffe                	sd	t6,504(sp)
    5f30:	c8756753          	0xc8756753
    5f34:	d0c9                	beqz	s1,5eb6 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0x9c>
    5f36:	d8d1                	beqz	s1,5eca <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0xb0>
    5f38:	e7d9                	bnez	a5,5fc6 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.349+0x8a>
    5f3a:	fffe                	sd	t6,504(sp)

0000000000005f3c <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.349>:
    5f3c:	2000                	fld	fs0,0(s0)
    5f3e:	225f df82 8204      	0x8204df82225f
    5f44:	0844                	addi	s1,sp,20
    5f46:	1106041b          	addiw	s0,a2,272
    5f4a:	ac81                	j	619a <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.352+0x2e>
    5f4c:	800e                	c.mv	zero,gp
    5f4e:	0b2835ab          	0xb2835ab
    5f52:	e080                	sd	s0,0(s1)
    5f54:	01081903          	lh	s2,16(a6) # ffffffffffff9010 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xfffffffffffe9ea0>
    5f58:	2f04                	fld	fs1,24(a4)
    5f5a:	3404                	fld	fs1,40(s0)
    5f5c:	0704                	addi	s1,sp,896
    5f5e:	06070103          	lb	sp,96(a4) # 1a060 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xaef0>
    5f62:	500a1107          	0x500a1107
    5f66:	5507120f          	0x5507120f
    5f6a:	1c040307          	0x1c040307
    5f6e:	090a                	slli	s2,s2,0x2
    5f70:	07030803          	lb	a6,112(t1) # ffffffffffffa070 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xfffffffffffeaf00>
    5f74:	03030203          	lb	tp,48(t1)
    5f78:	05040c03          	lb	s8,80(s0) # 12050 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x2ee0>
    5f7c:	01060b03          	lb	s6,16(a2)
    5f80:	150e                	slli	a0,a0,0x23
    5f82:	3a05                	addiw	s4,s4,-31
    5f84:	06071103          	lh	sp,96(a4)
    5f88:	1005                	c.nop	-31
    5f8a:	02075707          	0x2075707
    5f8e:	500d1507          	0x500d1507
    5f92:	4304                	lw	s1,0(a4)
    5f94:	01032d03          	lw	s10,16(t1)
    5f98:	1104                	addi	s1,sp,160
    5f9a:	0f06                	slli	t5,t5,0x1
    5f9c:	3a0c                	fld	fa1,48(a2)
    5f9e:	1d04                	addi	s1,sp,688
    5fa0:	5f25                	li	t5,-23
    5fa2:	6d20                	ld	s0,88(a0)
    5fa4:	6a04                	ld	s1,16(a2)
    5fa6:	8025                	srli	s0,s0,0x9
    5fa8:	05c8                	addi	a0,sp,708
    5faa:	b082                	fsd	ft0,96(sp)
    5fac:	82061a03          	lh	s4,-2016(a2)
    5fb0:	03fd                	addi	t2,t2,31
    5fb2:	0759                	addi	a4,a4,22
    5fb4:	0b15                	addi	s6,s6,5
    5fb6:	0c140917          	auipc	s2,0xc140
    5fba:	0c14                	addi	a3,sp,528
    5fbc:	066a                	slli	a2,a2,0x1a
    5fbe:	060a                	slli	a2,a2,0x2
    5fc0:	061a                	slli	a2,a2,0x6
    5fc2:	0759                	addi	a4,a4,22
    5fc4:	0a46052b          	0xa46052b
    5fc8:	042c                	addi	a1,sp,520
    5fca:	040c                	addi	a1,sp,512
    5fcc:	0301                	addi	t1,t1,0
    5fce:	0b31                	addi	s6,s6,12
    5fd0:	042c                	addi	a1,sp,520
    5fd2:	061a                	slli	a2,a2,0x6
    5fd4:	ac80030b          	0xac80030b
    5fd8:	0a06                	slli	s4,s4,0x1
    5fda:	2106                	fld	ft2,64(sp)
    5fdc:	2d044c3f 3c087403 	0x3c0874032d044c3f
    5fe4:	3c030f03          	lb	t5,960(t1)
    5fe8:	2b083807          	fld	fa6,688(a6)
    5fec:	8205                	srli	a2,a2,0x1
    5fee:	11ff                	0x11ff
    5ff0:	0818                	addi	a4,sp,16
    5ff2:	032d112f          	0x32d112f
    5ff6:	1020                	addi	s0,sp,40
    5ff8:	0f21                	addi	t5,t5,8
    5ffa:	8c80                	0x8c80
    5ffc:	8204                	0x8204
    5ffe:	150b1997          	auipc	s3,0x150b1
    6002:	9488                	0x9488
    6004:	2f05                	addiw	t5,t5,1
    6006:	3b05                	addiw	s6,s6,-31
    6008:	180e0207          	0x180e0207
    600c:	8009                	srli	s0,s0,0x2
    600e:	0c742db3          	0xc742db3
    6012:	d680                	sw	s0,40(a3)
    6014:	0c1a                	slli	s8,s8,0x6
    6016:	8005                	srli	s0,s0,0x1
    6018:	05ff                	0x5ff
    601a:	df80                	sw	s0,56(a5)
    601c:	ee0c                	sd	a1,24(a2)
    601e:	030d                	addi	t1,t1,3
    6020:	8d84                	0x8d84
    6022:	81093703          	ld	a4,-2032(s2) # c1457c6 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xc136656>
    6026:	145c                	addi	a5,sp,548
    6028:	b880                	fsd	fs0,48(s1)
    602a:	8008                	0x8008
    602c:	03382acb          	fnmsub.d	fs5,fa6,fs3,ft0,rdn
    6030:	060a                	slli	a2,a2,0x2
    6032:	0838                	addi	a4,sp,24
    6034:	0846                	slli	a6,a6,0x11
    6036:	060c                	addi	a1,sp,768
    6038:	0b74                	addi	a3,sp,412
    603a:	031e                	slli	t1,t1,0x7
    603c:	045a                	slli	s0,s0,0x16
    603e:	0959                	addi	s2,s2,22
    6040:	8380                	0x8380
    6042:	1c18                	addi	a4,sp,560
    6044:	160a                	slli	a2,a2,0x22
    6046:	4c09                	li	s8,2
    6048:	8004                	0x8004
    604a:	068a                	slli	a3,a3,0x2
    604c:	170ca4ab          	0x170ca4ab
    6050:	3104                	fld	fs1,32(a0)
    6052:	04a1                	addi	s1,s1,8
    6054:	da81                	beqz	a3,5f64 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.349+0x28>
    6056:	0726                	slli	a4,a4,0x9
    6058:	050c                	addi	a1,sp,640
    605a:	8005                	srli	s0,s0,0x1
    605c:	11a5                	addi	gp,gp,-23
    605e:	6d81                	0x6d81
    6060:	7810                	ld	a2,48(s0)
    6062:	2a28                	fld	fa0,80(a2)
    6064:	4c06                	lw	s8,64(sp)
    6066:	8004                	0x8004
    6068:	048d                	addi	s1,s1,3
    606a:	be80                	fsd	fs0,56(a3)
    606c:	0f031b03          	lh	s6,240(t1)
    6070:	                	c.nop	3

0000000000006071 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.350>:
    6071:	0600                	addi	s0,sp,768
    6073:	0101                	addi	sp,sp,0
    6075:	02040103          	lb	sp,32(s0)
    6079:	0808                	addi	a0,sp,16
    607b:	0209                	addi	tp,tp,2
    607d:	050a                	slli	a0,a0,0x2
    607f:	040e020b          	0x40e020b
    6083:	0110                	addi	a2,sp,128
    6085:	0211                	addi	tp,tp,4
    6087:	0512                	slli	a0,a0,0x4
    6089:	01141113          	slli	sp,s0,0x11
    608d:	0215                	addi	tp,tp,5
    608f:	0d190217          	auipc	tp,0xd190
    6093:	051c                	addi	a5,sp,640
    6095:	081d                	addi	a6,a6,7
    6097:	0124                	addi	s1,sp,136
    6099:	036a                	slli	t1,t1,0x1a
    609b:	02bc026b          	0x2bc026b
    609f:	02d1                	addi	t0,t0,20
    60a1:	0cd4                	addi	a3,sp,596
    60a3:	09d5                	addi	s3,s3,21
    60a5:	02d6                	slli	t0,t0,0x15
    60a7:	01da02d7          	0x1da02d7
    60ab:	05e0                	addi	s0,sp,716
    60ad:	02e1                	addi	t0,t0,24
    60af:	02e8                	addi	a0,sp,332
    60b1:	20ee                	fld	ft1,216(sp)
    60b3:	04f0                	addi	a2,sp,588
    60b5:	02f8                	addi	a4,sp,332
    60b7:	02f9                	addi	t0,t0,30
    60b9:	02fa                	slli	t0,t0,0x1e
    60bb:	          	0x270c01fb

00000000000060bd <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.351>:
    60bd:	270c                	fld	fa1,8(a4)
    60bf:	4f4e3e3b          	0x4f4e3e3b
    60c3:	9f9e9e8f          	0x9f9e9e8f
    60c7:	0706                	slli	a4,a4,0x1
    60c9:	3609                	addiw	a2,a2,-30
    60cb:	3e3d                	addiw	t3,t3,-17
    60cd:	f356                	sd	s5,416(sp)
    60cf:	d1d0                	sw	a2,36(a1)
    60d1:	1404                	addi	s1,sp,544
    60d3:	3618                	fld	fa4,40(a2)
    60d5:	7f575637          	lui	a2,0x7f575
    60d9:	aeaa                	fsd	fa0,344(sp)
    60db:	e035bdaf          	amomaxu.d	s11,gp,(a1)
    60df:	8712                	mv	a4,tp
    60e1:	8e89                	sub	a3,a3,a0
    60e3:	049e                	slli	s1,s1,0x7
    60e5:	0e0d                	addi	t3,t3,3
    60e7:	1211                	addi	tp,tp,-28
    60e9:	3129                	addiw	sp,sp,-22
    60eb:	3a34                	fld	fa3,112(a2)
    60ed:	4645                	li	a2,17
    60ef:	4a49                	li	s4,18
    60f1:	4f4e                	lw	t5,208(sp)
    60f3:	6564                	ld	s1,200(a0)
    60f5:	b65c                	fsd	fa5,168(a2)
    60f7:	071c1bb7          	lui	s7,0x71c1
    60fb:	0a08                	addi	a0,sp,272
    60fd:	3617140b          	0x3617140b
    6101:	3a39                	addiw	s4,s4,-18
    6103:	a9a8                	fsd	fa0,80(a1)
    6105:	d9d8                	sw	a4,52(a1)
    6107:	3709                	addiw	a4,a4,-30
    6109:	9190                	0x9190
    610b:	07a8                	addi	a0,sp,968
    610d:	3b0a                	fld	fs6,160(sp)
    610f:	663e                	ld	a2,456(sp)
    6111:	8f69                	and	a4,a4,a0
    6113:	6f92                	ld	t6,256(sp)
    6115:	ee5f 5aef 9a62      	0x9a625aefee5f
    611b:	5528279b          	0x5528279b
    611f:	a09d                	j	6185 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.352+0x19>
    6121:	a3a1                	j	6669 <_ZN4core7unicode12unicode_data15grapheme_extend7OFFSETS17h21859fdd3179f732E+0x26d>
    6123:	a7a4                	fsd	fs1,72(a5)
    6125:	ada8                	fsd	fa0,88(a1)
    6127:	bcba                	fsd	fa4,120(sp)
    6129:	06c4                	addi	s1,sp,836
    612b:	1d150c0b          	0x1d150c0b
    612f:	3f3a                	fld	ft10,424(sp)
    6131:	5145                	li	sp,-15
    6133:	a7a6                	fsd	fs1,456(sp)
    6135:	cdcc                	sw	a1,28(a1)
    6137:	07a0                	addi	s0,sp,968
    6139:	1a19                	addi	s4,s4,-26
    613b:	2522                	fld	fa0,8(sp)
    613d:	3f3e                	fld	ft10,488(sp)
    613f:	c6c5                	beqz	a3,61e7 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.352+0x7b>
    6141:	2004                	fld	fs1,0(s0)
    6143:	28262523          	sw	sp,650(a2) # 7f57528a <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x7f56611a>
    6147:	483a3833          	0x483a3833
    614b:	4c4a                	lw	s8,144(sp)
    614d:	5350                	lw	a2,36(a4)
    614f:	5655                	li	a2,-11
    6151:	5a58                	lw	a4,52(a2)
    6153:	5e5c                	lw	a5,60(a2)
    6155:	6360                	ld	s0,192(a4)
    6157:	6665                	lui	a2,0x19
    6159:	7d78736b          	0x7d78736b
    615d:	8a7f                	0x8a7f
    615f:	aaa4                	fsd	fs1,80(a3)
    6161:	d0c0b0af          	0xd0c0b0af
    6165:	afae                	fsd	fa1,472(sp)
    6167:	cc79                	beqz	s0,6245 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.352+0xd9>
    6169:	6f6e                	ld	t5,216(sp)
    616b:	          	0x7b225e93

000000000000616c <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.352>:
    616c:	225e                	fld	ft4,464(sp)
    616e:	0403057b          	0x403057b
    6172:	032d                	addi	t1,t1,11
    6174:	0366                	slli	t1,t1,0x19
    6176:	2f01                	sext.w	t5,t5
    6178:	802e                	c.mv	zero,a1
    617a:	1d82                	slli	s11,s11,0x20
    617c:	1c0f3103          	ld	sp,448(t5) # 1a1c0 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xb050>
    6180:	2404                	fld	fs1,8(s0)
    6182:	1e09                	addi	t3,t3,-30
    6184:	2b05                	addiw	s6,s6,1
    6186:	4405                	li	s0,1
    6188:	0e04                	addi	s1,sp,784
    618a:	802a                	c.mv	zero,a0
    618c:	06aa                	slli	a3,a3,0xa
    618e:	0424                	addi	s1,sp,520
    6190:	0424                	addi	s1,sp,520
    6192:	0828                	addi	a0,sp,24
    6194:	0b34                	addi	a3,sp,408
    6196:	8001                	c.srli64	s0
    6198:	8190                	0x8190
    619a:	0a160937          	lui	s2,0xa160
    619e:	8008                	0x8008
    61a0:	3998                	fld	fa4,48(a1)
    61a2:	09086303          	lwu	t1,144(a6)
    61a6:	1630                	addi	a2,sp,808
    61a8:	2105                	addiw	sp,sp,1
    61aa:	01051b03          	lh	s6,16(a0) # 2185dfa <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x2176c8a>
    61ae:	3840                	fld	fs0,176(s0)
    61b0:	4b04                	lw	s1,16(a4)
    61b2:	2f05                	addiw	t5,t5,1
    61b4:	0a04                	addi	s1,sp,272
    61b6:	40070907          	0x40070907
    61ba:	2720                	fld	fs0,72(a4)
    61bc:	0c04                	addi	s1,sp,528
    61be:	3609                	addiw	a2,a2,-30
    61c0:	1a053a03          	ld	s4,416(a0)
    61c4:	070c0407          	0x70c0407
    61c8:	4950                	lw	a2,20(a0)
    61ca:	330d3337          	lui	t1,0x330d3
    61ce:	0a082e07          	flw	ft8,160(a6)
    61d2:	2681                	sext.w	a3,a3
    61d4:	4e52                	lw	t3,20(sp)
    61d6:	0828                	addi	a0,sp,24
    61d8:	562a                	lw	a2,168(sp)
    61da:	141c                	addi	a5,sp,544
    61dc:	044e0917          	auipc	s2,0x44e0
    61e0:	0f1e                	slli	t5,t5,0x7
    61e2:	07190e43          	fmadd.q	ft8,fs2,fa7,ft0,rne
    61e6:	060a                	slli	a2,a2,0x2
    61e8:	0848                	addi	a0,sp,20
    61ea:	0b750927          	0xb750927
    61ee:	062a413f 060a053b 	0x60a053b062a413f
    61f6:	0651                	addi	a2,a2,20
    61f8:	0501                	addi	a0,a0,0
    61fa:	0310                	addi	a2,sp,384
    61fc:	8005                	srli	s0,s0,0x1
    61fe:	481e628b          	0x481e628b
    6202:	0a08                	addi	a0,sp,272
    6204:	a680                	fsd	fs0,8(a3)
    6206:	225e                	fld	ft4,464(sp)
    6208:	0b45                	addi	s6,s6,17
    620a:	060a                	slli	a2,a2,0x2
    620c:	130d                	addi	t1,t1,-29
    620e:	0739                	addi	a4,a4,14
    6210:	360a                	fld	fa2,160(sp)
    6212:	042c                	addi	a1,sp,520
    6214:	8010                	0x8010
    6216:	3cc0                	fld	fs0,184(s1)
    6218:	5364                	lw	s1,100(a4)
    621a:	480c                	lw	a1,16(s0)
    621c:	0a09                	addi	s4,s4,2
    621e:	4546                	lw	a0,80(sp)
    6220:	5308481b          	0x5308481b
    6224:	391d                	addiw	s2,s2,-25
    6226:	0781                	addi	a5,a5,0
    6228:	0a46                	slli	s4,s4,0x11
    622a:	031d                	addi	t1,t1,7
    622c:	03374947          	fmsub.d	fs2,fa4,fs3,ft0,rmm
    6230:	080e                	slli	a6,a6,0x3
    6232:	060a                	slli	a2,a2,0x2
    6234:	0739                	addi	a4,a4,14
    6236:	810a                	mv	sp,sp
    6238:	1936                	slli	s2,s2,0x2d
    623a:	b780                	fsd	fs0,40(a5)
    623c:	0f01                	addi	t5,t5,0
    623e:	0d32                	slli	s10,s10,0xc
    6240:	75669b83          	lh	s7,1878(a3)
    6244:	8ac4800b          	0x8ac4800b
    6248:	84bc                	0x84bc
    624a:	82d18f2f          	0x82d18f2f
    624e:	82b9a147          	fmsub.d	ft2,fs3,fa1,fa6,rdn
    6252:	0739                	addi	a4,a4,14
    6254:	042a                	slli	s0,s0,0xa
    6256:	6002                	0x6002
    6258:	0a26                	slli	s4,s4,0x9
    625a:	0a46                	slli	s4,s4,0x11
    625c:	0528                	addi	a0,sp,648
    625e:	5bb08213          	addi	tp,ra,1467
    6262:	4b65                	li	s6,25
    6264:	3904                	fld	fs1,48(a0)
    6266:	05401107          	0x5401107
    626a:	970e020b          	0x970e020b
    626e:	08f8                	addi	a4,sp,92
    6270:	d684                	sw	s1,40(a3)
    6272:	092a                	slli	s2,s2,0xa
    6274:	f7a2                	sd	s0,488(sp)
    6276:	1f81                	addi	t6,t6,-32
    6278:	0331                	addi	t1,t1,12
    627a:	0411                	addi	s0,s0,4
    627c:	8108                	0x8108
    627e:	898c                	0x898c
    6280:	6b04                	ld	s1,16(a4)
    6282:	0d05                	addi	s10,s10,1
    6284:	10070903          	lb	s2,256(a4)
    6288:	f6806093          	ori	ra,zero,-152
    628c:	730a                	ld	t1,160(sp)
    628e:	6e08                	ld	a0,24(a2)
    6290:	9a804617          	auipc	a2,0x9a804
    6294:	0c14                	addi	a3,sp,528
    6296:	80190957          	0x80190957
    629a:	03478187          	0x3478187
    629e:	4285                	li	t0,1
    62a0:	5085150f          	0x5085150f
    62a4:	2dd5802b          	0x2dd5802b
    62a8:	02041a03          	lh	s4,32(s0)
    62ac:	7081                	lui	ra,0xfffe0
    62ae:	053a                	slli	a0,a0,0xe
    62b0:	8501                	c.srai64	a0
    62b2:	8000                	0x8000
    62b4:	044c29d7          	0x44c29d7
    62b8:	040a                	slli	s0,s0,0x2
    62ba:	8302                	jr	t1
    62bc:	4411                	li	s0,4
    62be:	3d4c                	fld	fa1,184(a0)
    62c0:	c280                	sw	s0,0(a3)
    62c2:	063c                	addi	a5,sp,776
    62c4:	0401                	addi	s0,s0,0
    62c6:	0555                	addi	a0,a0,21
    62c8:	8102341b          	0x8102341b
    62cc:	2c0e                	fld	fs8,192(sp)
    62ce:	6404                	ld	s1,8(s0)
    62d0:	560c                	lw	a1,40(a2)
    62d2:	800a                	c.mv	zero,sp
    62d4:	38ae                	fld	fa7,232(sp)
    62d6:	0d1d                	addi	s10,s10,7
    62d8:	042c                	addi	a1,sp,520
    62da:	0709                	addi	a4,a4,2
    62dc:	0e02                	c.slli64	t3
    62de:	8006                	c.mv	zero,ra
    62e0:	839a                	mv	t2,t1
    62e2:	08d8                	addi	a4,sp,84
    62e4:	030d                	addi	t1,t1,3
    62e6:	030d                	addi	t1,t1,3
    62e8:	0c74                	addi	a3,sp,540
    62ea:	0759                	addi	a4,a4,22
    62ec:	140c                	addi	a1,sp,544
    62ee:	040c                	addi	a1,sp,512
    62f0:	0838                	addi	a4,sp,24
    62f2:	060a                	slli	a2,a2,0x2
    62f4:	0828                	addi	a0,sp,24
    62f6:	4e22                	lw	t3,8(sp)
    62f8:	5481                	li	s1,-32
    62fa:	150c                	addi	a1,sp,672
    62fc:	07050303          	lb	t1,112(a0)
    6300:	1909                	addi	s2,s2,-30
    6302:	03090707          	0x3090707
    6306:	070d                	addi	a4,a4,3
    6308:	8029                	srli	s0,s0,0xa
    630a:	840a25cb          	0x840a25cb
    630e:	                	ld	s8,64(sp)

000000000000630f <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.353>:
    630f:	696c                	ld	a1,208(a0)
    6311:	7262                	ld	tp,56(sp)
    6313:	7261                	lui	tp,0xffff8
    6315:	2f79                	addiw	t5,t5,30
    6317:	65726f63          	bltu	tp,s7,6975 <_ZN4core7unicode12unicode_data15grapheme_extend7OFFSETS17h21859fdd3179f732E+0x579>
    631b:	6372732f          	0x6372732f
    631f:	696e752f          	0x696e752f
    6323:	65646f63          	bltu	s0,s6,6981 <_ZN4core7unicode12unicode_data15grapheme_extend7OFFSETS17h21859fdd3179f732E+0x585>
    6327:	696e752f          	0x696e752f
    632b:	65646f63          	bltu	s0,s6,6989 <_ZN4core7unicode12unicode_data15grapheme_extend7OFFSETS17h21859fdd3179f732E+0x58d>
    632f:	645f 7461 2e61      	0x2e617461645f
    6335:	7372                	ld	t1,312(sp)
	...

0000000000006338 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.357>:
    6338:	0000630f          	0x630f
    633c:	0000                	unimp
    633e:	0000                	unimp
    6340:	0028                	addi	a0,sp,8
    6342:	0000                	unimp
    6344:	0000                	unimp
    6346:	0000                	unimp
    6348:	0000004b          	fnmsub.s	ft0,ft0,ft0,ft0,rne
    634c:	0028                	addi	a0,sp,8
	...

0000000000006350 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.358>:
    6350:	0000630f          	0x630f
    6354:	0000                	unimp
    6356:	0000                	unimp
    6358:	0028                	addi	a0,sp,8
    635a:	0000                	unimp
    635c:	0000                	unimp
    635e:	0000                	unimp
    6360:	00000057          	0x57
    6364:	0016                	c.slli	zero,0x5
	...

0000000000006368 <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.359>:
    6368:	0000630f          	0x630f
    636c:	0000                	unimp
    636e:	0000                	unimp
    6370:	0028                	addi	a0,sp,8
    6372:	0000                	unimp
    6374:	0000                	unimp
    6376:	0000                	unimp
    6378:	0052                	c.slli	zero,0x14
    637a:	0000                	unimp
    637c:	003e                	c.slli	zero,0xf
	...

0000000000006380 <_ZN4core7unicode12unicode_data15grapheme_extend17SHORT_OFFSET_RUNS17hff6f6dc47b6092b7E>:
    6380:	0300                	addi	s0,sp,384
    6382:	0000                	unimp
    6384:	00200483          	lb	s1,2(zero) # 2 <_start+0x2>
    6388:	0591                	addi	a1,a1,4
    638a:	0060                	addi	s0,sp,12
    638c:	135d                	addi	t1,t1,-9
    638e:	00a0                	addi	s0,sp,72
    6390:	1712                	slli	a4,a4,0x24
    6392:	1ea0                	addi	s0,sp,888
    6394:	200c                	fld	fa1,0(s0)
    6396:	1ee0                	addi	s0,sp,892
    6398:	2b202cef          	jal	s9,864a <_ZN12tornado_user10HEAP_SPACE17hcda5f67e49da29e7E+0x1612>
    639c:	302a                	fld	ft0,168(sp)
    639e:	2ba0                	fld	fs0,80(a5)
    63a0:	2c60a66f          	jal	a2,10666 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x14f6>
    63a4:	a802                	fsd	ft0,16(sp)
    63a6:	2ce0                	fld	fs0,216(s1)
    63a8:	fb1e                	sd	t2,432(sp)
    63aa:	2de0                	fld	fs0,216(a1)
    63ac:	fe00                	sd	s0,56(a2)
    63ae:	35a0                	fld	fs0,104(a1)
    63b0:	ff9e                	sd	t2,504(sp)
    63b2:	35e0                	fld	fs0,232(a1)
    63b4:	01fd                	addi	gp,gp,31
    63b6:	3661                	addiw	a2,a2,-8
    63b8:	0a01                	addi	s4,s4,0
    63ba:	36a1                	addiw	a3,a3,-24
    63bc:	0d24                	addi	s1,sp,664
    63be:	3761                	addiw	a4,a4,-8
    63c0:	38e10eab          	0x38e10eab
    63c4:	3921182f          	0x3921182f
    63c8:	1c30                	addi	a2,sp,568
    63ca:	4661                	li	a2,24
    63cc:	4aa11ef3          	csrrw	t4,0x4aa,sp
    63d0:	6af0                	ld	a2,208(a3)
    63d2:	4e61                	li	t3,24
    63d4:	4ea16f4f          	fnmadd.q	ft10,ft2,fa0,fs1,unknown
    63d8:	bc9d                	j	5e4e <.Lanon.7cbaf4c7391142b3aa5e9b648bdd2533.348+0x34>
    63da:	4f21                	li	t5,8
    63dc:	d165                	beqz	a0,63bc <_ZN4core7unicode12unicode_data15grapheme_extend17SHORT_OFFSET_RUNS17hff6f6dc47b6092b7E+0x3c>
    63de:	4fe1                	li	t6,24
    63e0:	da00                	sw	s0,48(a2)
    63e2:	5021                	c.li	zero,-24
    63e4:	e000                	sd	s0,0(s0)
    63e6:	51e1                	li	gp,-8
    63e8:	e130                	sd	a2,64(a0)
    63ea:	5361                	li	t1,-8
    63ec:	e2ec                	sd	a1,192(a3)
    63ee:	54a1                	li	s1,-24
    63f0:	e8d0                	sd	a2,144(s1)
    63f2:	54e1                	li	s1,-8
    63f4:	0020                	addi	s0,sp,8
    63f6:	552e                	lw	a0,232(sp)
    63f8:	01f0                	addi	a2,sp,204
    63fa:	  	0x2d000700700055bf

00000000000063fc <_ZN4core7unicode12unicode_data15grapheme_extend7OFFSETS17h21859fdd3179f732E>:
    63fc:	7000                	ld	s0,32(s0)
    63fe:	0700                	addi	s0,sp,896
    6400:	2d00                	fld	fs0,24(a0)
    6402:	0101                	addi	sp,sp,0
    6404:	0201                	addi	tp,tp,0
    6406:	0201                	addi	tp,tp,0
    6408:	0101                	addi	sp,sp,0
    640a:	0b48                	addi	a0,sp,404
    640c:	1530                	addi	a2,sp,680
    640e:	0110                	addi	a2,sp,128
    6410:	0765                	addi	a4,a4,25
    6412:	0602                	c.slli64	a2
    6414:	0202                	c.slli64	tp
    6416:	0401                	addi	s0,s0,0
    6418:	1b1e0123          	sb	a7,418(t3) # 461f7088 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x461e7f18>
    641c:	093a0b5b          	0x93a0b5b
    6420:	0109                	addi	sp,sp,2
    6422:	0418                	addi	a4,sp,512
    6424:	0901                	addi	s2,s2,0
    6426:	0301                	addi	t1,t1,0
    6428:	0501                	addi	a0,a0,0
    642a:	0f77032b          	0xf77032b
    642e:	2001                	0x2001
    6430:	01010137          	lui	sp,0x1010
    6434:	0804                	addi	s1,sp,16
    6436:	0104                	addi	s1,sp,128
    6438:	020a0703          	lb	a4,32(s4) # 110020 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x100eb0>
    643c:	011d                	addi	sp,sp,7
    643e:	013a                	slli	sp,sp,0xe
    6440:	0101                	addi	sp,sp,0
    6442:	0402                	c.slli64	s0
    6444:	0108                	addi	a0,sp,128
    6446:	0109                	addi	sp,sp,2
    6448:	020a                	slli	tp,tp,0x2
    644a:	011a                	slli	sp,sp,0x6
    644c:	0202                	c.slli64	tp
    644e:	0139                	addi	sp,sp,14
    6450:	0204                	addi	s1,sp,256
    6452:	0204                	addi	s1,sp,256
    6454:	0302                	c.slli64	t1
    6456:	021e0103          	lb	sp,33(t3)
    645a:	020b0103          	lb	sp,32(s6)
    645e:	0139                	addi	sp,sp,14
    6460:	0504                	addi	s1,sp,640
    6462:	0201                	addi	tp,tp,0
    6464:	0104                	addi	s1,sp,128
    6466:	0214                	addi	a3,sp,256
    6468:	0616                	slli	a2,a2,0x5
    646a:	0101                	addi	sp,sp,0
    646c:	013a                	slli	sp,sp,0xe
    646e:	0201                	addi	tp,tp,0
    6470:	0401                	addi	s0,s0,0
    6472:	0108                	addi	a0,sp,128
    6474:	020a0307          	0x20a0307
    6478:	011e                	slli	sp,sp,0x7
    647a:	0101013b          	addw	sp,sp,a6
    647e:	010c                	addi	a1,sp,128
    6480:	0109                	addi	sp,sp,2
    6482:	0128                	addi	a0,sp,136
    6484:	03390103          	lb	sp,51(s2) # 44e620f <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x44d709f>
    6488:	0305                	addi	t1,t1,1
    648a:	0401                	addi	s0,s0,0
    648c:	020b0207          	0x20b0207
    6490:	011d                	addi	sp,sp,7
    6492:	013a                	slli	sp,sp,0xe
    6494:	0102                	c.slli64	sp
    6496:	0102                	c.slli64	sp
    6498:	02050103          	lb	sp,32(a0)
    649c:	020b0207          	0x20b0207
    64a0:	021c                	addi	a5,sp,256
    64a2:	0239                	addi	tp,tp,14
    64a4:	0101                	addi	sp,sp,0
    64a6:	0402                	c.slli64	s0
    64a8:	0108                	addi	a0,sp,128
    64aa:	0109                	addi	sp,sp,2
    64ac:	020a                	slli	tp,tp,0x2
    64ae:	011d                	addi	sp,sp,7
    64b0:	0148                	addi	a0,sp,132
    64b2:	0104                	addi	s1,sp,128
    64b4:	0302                	c.slli64	t1
    64b6:	0101                	addi	sp,sp,0
    64b8:	0108                	addi	a0,sp,128
    64ba:	0151                	addi	sp,sp,20
    64bc:	0702                	c.slli64	a4
    64be:	080c                	addi	a1,sp,16
    64c0:	0162                	slli	sp,sp,0x18
    64c2:	0902                	c.slli64	s2
    64c4:	024a060b          	0x24a060b
    64c8:	0101011b          	addiw	sp,sp,16
    64cc:	0101                	addi	sp,sp,0
    64ce:	05010e37          	lui	t3,0x5010
    64d2:	0201                	addi	tp,tp,0
    64d4:	0b05                	addi	s6,s6,1
    64d6:	2401                	sext.w	s0,s0
    64d8:	0109                	addi	sp,sp,2
    64da:	0466                	slli	s0,s0,0x19
    64dc:	0601                	addi	a2,a2,0
    64de:	0201                	addi	tp,tp,0
    64e0:	0202                	c.slli64	tp
    64e2:	0219                	addi	tp,tp,6
    64e4:	0304                	addi	s1,sp,384
    64e6:	0410                	addi	a2,sp,512
    64e8:	010d                	addi	sp,sp,3
    64ea:	0202                	c.slli64	tp
    64ec:	0106                	slli	sp,sp,0x1
    64ee:	0300010f          	0x300010f
    64f2:	0300                	addi	s0,sp,384
    64f4:	031d                	addi	t1,t1,7
    64f6:	021d                	addi	tp,tp,7
    64f8:	021e                	slli	tp,tp,0x7
    64fa:	0240                	addi	s0,sp,260
    64fc:	0701                	addi	a4,a4,0
    64fe:	0108                	addi	a0,sp,128
    6500:	0b02                	c.slli64	s6
    6502:	0109                	addi	sp,sp,2
    6504:	032d                	addi	t1,t1,11
    6506:	01220277          	0x1220277
    650a:	0376                	slli	t1,t1,0x1d
    650c:	0204                	addi	s1,sp,256
    650e:	0109                	addi	sp,sp,2
    6510:	0306                	slli	t1,t1,0x1
    6512:	010202db          	0x10202db
    6516:	013a                	slli	sp,sp,0xe
    6518:	0701                	addi	a4,a4,0
    651a:	0101                	addi	sp,sp,0
    651c:	0101                	addi	sp,sp,0
    651e:	0802                	c.slli64	a6
    6520:	0a06                	slli	s4,s4,0x1
    6522:	0102                	c.slli64	sp
    6524:	1130                	addi	a2,sp,168
    6526:	0730043f 01050101 	0x10501010730043f
    652e:	0928                	addi	a0,sp,152
    6530:	020c                	addi	a1,sp,256
    6532:	0420                	addi	s0,sp,520
    6534:	0202                	c.slli64	tp
    6536:	0301                	addi	t1,t1,0
    6538:	0138                	addi	a4,sp,136
    653a:	0201                	addi	tp,tp,0
    653c:	03010103          	lb	sp,48(sp) # 1010030 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x1000ec0>
    6540:	083a                	slli	a6,a6,0xe
    6542:	0202                	c.slli64	tp
    6544:	0398                	addi	a4,sp,448
    6546:	0d01                	addi	s10,s10,0
    6548:	0701                	addi	a4,a4,0
    654a:	0104                	addi	s1,sp,128
    654c:	0106                	slli	sp,sp,0x1
    654e:	3ac60203          	lb	tp,940(a2) # ffffffff9a80a63c <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0xffffffff9a7fb4cc>
    6552:	0501                	addi	a0,a0,0
    6554:	0100                	addi	s0,sp,128
    6556:	030021c3          	fmadd.d	ft3,ft0,fa6,ft0,rdn
    655a:	018d                	addi	gp,gp,3
    655c:	2060                	fld	fs0,192(s0)
    655e:	0600                	addi	s0,sp,768
    6560:	0269                	addi	tp,tp,26
    6562:	0400                	addi	s0,sp,512
    6564:	0a01                	addi	s4,s4,0
    6566:	0220                	addi	s0,sp,264
    6568:	0250                	addi	a2,sp,260
    656a:	0100                	addi	s0,sp,128
    656c:	01040103          	lb	sp,16(s0)
    6570:	0219                	addi	tp,tp,6
    6572:	0105                	addi	sp,sp,1
    6574:	121a0297          	auipc	t0,0x121a0
    6578:	010d                	addi	sp,sp,3
    657a:	0826                	slli	a6,a6,0x9
    657c:	0b19                	addi	s6,s6,6
    657e:	032e                	slli	t1,t1,0xb
    6580:	0130                	addi	a2,sp,136
    6582:	0402                	c.slli64	s0
    6584:	0202                	c.slli64	tp
    6586:	06430127          	0x6430127
    658a:	0202                	c.slli64	tp
    658c:	0202                	c.slli64	tp
    658e:	010c                	addi	a1,sp,128
    6590:	0108                	addi	a0,sp,128
    6592:	0133012f          	0x133012f
    6596:	0301                	addi	t1,t1,0
    6598:	0202                	c.slli64	tp
    659a:	0205                	addi	tp,tp,1
    659c:	0101                	addi	sp,sp,0
    659e:	022a                	slli	tp,tp,0xa
    65a0:	0108                	addi	a0,sp,128
    65a2:	01ee                	slli	gp,gp,0x1b
    65a4:	0102                	c.slli64	sp
    65a6:	0104                	addi	s1,sp,128
    65a8:	0100                	addi	s0,sp,128
    65aa:	1000                	addi	s0,sp,32
    65ac:	1010                	addi	a2,sp,32
    65ae:	0200                	addi	s0,sp,256
    65b0:	0100                	addi	s0,sp,128
    65b2:	01e2                	slli	gp,gp,0x18
    65b4:	0595                	addi	a1,a1,5
    65b6:	0300                	addi	s0,sp,384
    65b8:	0201                	addi	tp,tp,0
    65ba:	0405                	addi	s0,s0,1
    65bc:	0328                	addi	a0,sp,392
    65be:	0104                	addi	s1,sp,128
    65c0:	02a5                	addi	t0,t0,9
    65c2:	0400                	addi	s0,sp,512
    65c4:	0200                	addi	s0,sp,256
    65c6:	0b99                	addi	s7,s7,6
    65c8:	01b0                	addi	a2,sp,200
    65ca:	0f36                	slli	t5,t5,0xd
    65cc:	0338                	addi	a4,sp,392
    65ce:	0431                	addi	s0,s0,12
    65d0:	0202                	c.slli64	tp
    65d2:	0345                	addi	t1,t1,17
    65d4:	0524                	addi	s1,sp,648
    65d6:	0801                	addi	a6,a6,0
    65d8:	013e                	slli	sp,sp,0xf
    65da:	020c                	addi	a1,sp,256
    65dc:	0934                	addi	a3,sp,152
    65de:	040a                	slli	s0,s0,0x2
    65e0:	0102                	c.slli64	sp
    65e2:	035f 0102 0201      	0x2010102035f
    65e8:	0106                	slli	sp,sp,0x1
    65ea:	01a0                	addi	s0,sp,200
    65ec:	02150803          	lb	a6,33(a0)
    65f0:	0239                	addi	tp,tp,14
    65f2:	0101                	addi	sp,sp,0
    65f4:	0101                	addi	sp,sp,0
    65f6:	0116                	slli	sp,sp,0x5
    65f8:	070e                	slli	a4,a4,0x3
    65fa:	08c30503          	lb	a0,140(t1) # 330d308c <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x330c3f1c>
    65fe:	0302                	c.slli64	t1
    6600:	0101                	addi	sp,sp,0
    6602:	01510117          	auipc	sp,0x1510
    6606:	0602                	c.slli64	a2
    6608:	0101                	addi	sp,sp,0
    660a:	0102                	c.slli64	sp
    660c:	0201                	addi	tp,tp,0
    660e:	0201                	addi	tp,tp,0
    6610:	040201eb          	0x40201eb
    6614:	0206                	slli	tp,tp,0x1
    6616:	0201                	addi	tp,tp,0
    6618:	0855021b          	addiw	tp,a0,133
    661c:	0102                	c.slli64	sp
    661e:	0201                	addi	tp,tp,0
    6620:	016a                	slli	sp,sp,0x1a
    6622:	0101                	addi	sp,sp,0
    6624:	0602                	c.slli64	a2
    6626:	0101                	addi	sp,sp,0
    6628:	0365                	addi	t1,t1,25
    662a:	0402                	c.slli64	s0
    662c:	0501                	addi	a0,a0,0
    662e:	0900                	addi	s0,sp,144
    6630:	0201                	addi	tp,tp,0
    6632:	01f5                	addi	gp,gp,29
    6634:	020a                	slli	tp,tp,0x2
    6636:	0101                	addi	sp,sp,0
    6638:	0104                	addi	s1,sp,128
    663a:	0490                	addi	a2,sp,576
    663c:	0202                	c.slli64	tp
    663e:	0104                	addi	s1,sp,128
    6640:	0a20                	addi	s0,sp,280
    6642:	0628                	addi	a0,sp,776
    6644:	0402                	c.slli64	s0
    6646:	0108                	addi	a0,sp,128
    6648:	0609                	addi	a2,a2,2
    664a:	0302                	c.slli64	t1
    664c:	0d2e                	slli	s10,s10,0xb
    664e:	0201                	addi	tp,tp,0
    6650:	0700                	addi	s0,sp,896
    6652:	0601                	addi	a2,a2,0
    6654:	0101                	addi	sp,sp,0
    6656:	1652                	slli	a2,a2,0x34
    6658:	0702                	c.slli64	a4
    665a:	0201                	addi	tp,tp,0
    665c:	0201                	addi	tp,tp,0
    665e:	067a                	slli	a2,a2,0x1e
    6660:	02010103          	lb	sp,32(sp) # 1516622 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE+0x15074b2>
    6664:	0701                	addi	a4,a4,0
    6666:	0101                	addi	sp,sp,0
    6668:	0248                	addi	a0,sp,260
    666a:	01010103          	lb	sp,16(sp)
    666e:	0200                	addi	s0,sp,256
    6670:	0500                	addi	s0,sp,640
    6672:	0100073b          	addw	a4,zero,a6
    6676:	0151043f 01000200 	0x10002000151043f
    667e:	0301                	addi	t1,t1,0
    6680:	0504                	addi	s1,sp,640
    6682:	0808                	addi	a0,sp,16
    6684:	0702                	c.slli64	a4
    6686:	041e                	slli	s0,s0,0x7
    6688:	0394                	addi	a3,sp,448
    668a:	3700                	fld	fs0,40(a4)
    668c:	3204                	fld	fs1,32(a2)
    668e:	0108                	addi	a0,sp,128
    6690:	010e                	slli	sp,sp,0x3
    6692:	0516                	slli	a0,a0,0x5
    6694:	0f01                	addi	t5,t5,0
    6696:	0700                	addi	s0,sp,896
    6698:	1101                	addi	sp,sp,-32
    669a:	0702                	c.slli64	a4
    669c:	0201                	addi	tp,tp,0
    669e:	0501                	addi	a0,a0,0
    66a0:	0700                	addi	s0,sp,896
    66a2:	0400                	addi	s0,sp,512
    66a4:	0700                	addi	s0,sp,896
    66a6:	076d                	addi	a4,a4,27
    66a8:	6000                	ld	s0,0(s0)
    66aa:	f080                	sd	s0,32(s1)
	...

Disassembly of section .bss:

0000000000007000 <_ZN75_$LT$tornado_user..excutor..EXECUTOR$u20$as$u20$core..ops..deref..Deref$GT$5deref11__stability4LAZY17h53ae8f4cd44a05c3E>:
	...

0000000000007038 <_ZN12tornado_user10HEAP_SPACE17hcda5f67e49da29e7E>:
	...

000000000000f038 <_ZN12tornado_user4HEAP17h1339c640c584f725E>:
	...

Disassembly of section .sbss:

000000000000f160 <_ZN12tornado_user4task10UserTaskId8generate7COUNTER17h13230ff31d38f7aaE>:
	...

000000000000f168 <_ZN12tornado_user16ADDRESS_SPACE_ID17hd7ca12c82c04119fE>:
	...

000000000000f170 <_ZN12tornado_user16SHARED_RAW_TABLE17hf7654d85d7222c2fE>:
	...

Disassembly of section .riscv.attributes:

0000000000000000 <.riscv.attributes>:
   0:	2a41                	addiw	s4,s4,16
   2:	0000                	unimp
   4:	7200                	ld	s0,32(a2)
   6:	7369                	lui	t1,0xffffa
   8:	01007663          	bgeu	zero,a6,14 <.LBB2_3+0xc>
   c:	0020                	addi	s0,sp,8
   e:	0000                	unimp
  10:	1004                	addi	s1,sp,32
  12:	7205                	lui	tp,0xfffe1
  14:	3676                	fld	fa2,376(sp)
  16:	6934                	ld	a3,80(a0)
  18:	7032                	0x7032
  1a:	5f30                	lw	a2,120(a4)
  1c:	326d                	addiw	tp,tp,-5
  1e:	3070                	fld	fa2,224(s0)
  20:	615f 7032 5f30      	0x5f307032615f
  26:	30703263          	0x30703263
  2a:	4100                	lw	s0,0(a0)
  2c:	002a                	c.slli	zero,0xa
  2e:	0000                	unimp
  30:	6972                	ld	s2,280(sp)
  32:	00766373          	csrrsi	t1,0x7,12
  36:	2001                	0x2001
  38:	0000                	unimp
  3a:	0400                	addi	s0,sp,512
  3c:	0510                	addi	a2,sp,640
  3e:	7672                	ld	a2,312(sp)
  40:	3436                	fld	fs0,360(sp)
  42:	3269                	addiw	tp,tp,-6
  44:	3070                	fld	fa2,224(s0)
  46:	6d5f 7032 5f30      	0x5f3070326d5f
  4c:	3261                	addiw	tp,tp,-8
  4e:	3070                	fld	fa2,224(s0)
  50:	635f 7032 0030      	0x307032635f
  56:	2a41                	addiw	s4,s4,16
  58:	0000                	unimp
  5a:	7200                	ld	s0,32(a2)
  5c:	7369                	lui	t1,0xffffa
  5e:	01007663          	bgeu	zero,a6,6a <.LBB2_6+0x12>
  62:	0020                	addi	s0,sp,8
  64:	0000                	unimp
  66:	1004                	addi	s1,sp,32
  68:	7205                	lui	tp,0xfffe1
  6a:	3676                	fld	fa2,376(sp)
  6c:	6934                	ld	a3,80(a0)
  6e:	7032                	0x7032
  70:	5f30                	lw	a2,120(a4)
  72:	326d                	addiw	tp,tp,-5
  74:	3070                	fld	fa2,224(s0)
  76:	615f 7032 5f30      	0x5f307032615f
  7c:	30703263          	0x30703263
  80:	4100                	lw	s0,0(a0)
  82:	002a                	c.slli	zero,0xa
  84:	0000                	unimp
  86:	6972                	ld	s2,280(sp)
  88:	00766373          	csrrsi	t1,0x7,12
  8c:	2001                	0x2001
  8e:	0000                	unimp
  90:	0400                	addi	s0,sp,512
  92:	0510                	addi	a2,sp,640
  94:	7672                	ld	a2,312(sp)
  96:	3436                	fld	fs0,360(sp)
  98:	3269                	addiw	tp,tp,-6
  9a:	3070                	fld	fa2,224(s0)
  9c:	6d5f 7032 5f30      	0x5f3070326d5f
  a2:	3261                	addiw	tp,tp,-8
  a4:	3070                	fld	fa2,224(s0)
  a6:	635f 7032 0030      	0x307032635f
  ac:	2a41                	addiw	s4,s4,16
  ae:	0000                	unimp
  b0:	7200                	ld	s0,32(a2)
  b2:	7369                	lui	t1,0xffffa
  b4:	01007663          	bgeu	zero,a6,c0 <_ZN5alloc7raw_vec11finish_grow17h31ed548bca7fb5f1E.llvm.8801366308823686351+0x54>
  b8:	0020                	addi	s0,sp,8
  ba:	0000                	unimp
  bc:	1004                	addi	s1,sp,32
  be:	7205                	lui	tp,0xfffe1
  c0:	3676                	fld	fa2,376(sp)
  c2:	6934                	ld	a3,80(a0)
  c4:	7032                	0x7032
  c6:	5f30                	lw	a2,120(a4)
  c8:	326d                	addiw	tp,tp,-5
  ca:	3070                	fld	fa2,224(s0)
  cc:	615f 7032 5f30      	0x5f307032615f
  d2:	30703263          	0x30703263
  d6:	4100                	lw	s0,0(a0)
  d8:	002a                	c.slli	zero,0xa
  da:	0000                	unimp
  dc:	6972                	ld	s2,280(sp)
  de:	00766373          	csrrsi	t1,0x7,12
  e2:	2001                	0x2001
  e4:	0000                	unimp
  e6:	0400                	addi	s0,sp,512
  e8:	0510                	addi	a2,sp,640
  ea:	7672                	ld	a2,312(sp)
  ec:	3436                	fld	fs0,360(sp)
  ee:	3269                	addiw	tp,tp,-6
  f0:	3070                	fld	fa2,224(s0)
  f2:	6d5f 7032 5f30      	0x5f3070326d5f
  f8:	3261                	addiw	tp,tp,-8
  fa:	3070                	fld	fa2,224(s0)
  fc:	635f 7032 0030      	0x307032635f
 102:	2a41                	addiw	s4,s4,16
 104:	0000                	unimp
 106:	7200                	ld	s0,32(a2)
 108:	7369                	lui	t1,0xffffa
 10a:	01007663          	bgeu	zero,a6,116 <main+0x4a>
 10e:	0020                	addi	s0,sp,8
 110:	0000                	unimp
 112:	1004                	addi	s1,sp,32
 114:	7205                	lui	tp,0xfffe1
 116:	3676                	fld	fa2,376(sp)
 118:	6934                	ld	a3,80(a0)
 11a:	7032                	0x7032
 11c:	5f30                	lw	a2,120(a4)
 11e:	326d                	addiw	tp,tp,-5
 120:	3070                	fld	fa2,224(s0)
 122:	615f 7032 5f30      	0x5f307032615f
 128:	30703263          	0x30703263
 12c:	4100                	lw	s0,0(a0)
 12e:	002a                	c.slli	zero,0xa
 130:	0000                	unimp
 132:	6972                	ld	s2,280(sp)
 134:	00766373          	csrrsi	t1,0x7,12
 138:	2001                	0x2001
 13a:	0000                	unimp
 13c:	0400                	addi	s0,sp,512
 13e:	0510                	addi	a2,sp,640
 140:	7672                	ld	a2,312(sp)
 142:	3436                	fld	fs0,360(sp)
 144:	3269                	addiw	tp,tp,-6
 146:	3070                	fld	fa2,224(s0)
 148:	6d5f 7032 5f30      	0x5f3070326d5f
 14e:	3261                	addiw	tp,tp,-8
 150:	3070                	fld	fa2,224(s0)
 152:	635f 7032 0030      	0x307032635f
 158:	2a41                	addiw	s4,s4,16
 15a:	0000                	unimp
 15c:	7200                	ld	s0,32(a2)
 15e:	7369                	lui	t1,0xffffa
 160:	01007663          	bgeu	zero,a6,16c <main+0xa0>
 164:	0020                	addi	s0,sp,8
 166:	0000                	unimp
 168:	1004                	addi	s1,sp,32
 16a:	7205                	lui	tp,0xfffe1
 16c:	3676                	fld	fa2,376(sp)
 16e:	6934                	ld	a3,80(a0)
 170:	7032                	0x7032
 172:	5f30                	lw	a2,120(a4)
 174:	326d                	addiw	tp,tp,-5
 176:	3070                	fld	fa2,224(s0)
 178:	615f 7032 5f30      	0x5f307032615f
 17e:	30703263          	0x30703263
 182:	4100                	lw	s0,0(a0)
 184:	002a                	c.slli	zero,0xa
 186:	0000                	unimp
 188:	6972                	ld	s2,280(sp)
 18a:	00766373          	csrrsi	t1,0x7,12
 18e:	2001                	0x2001
 190:	0000                	unimp
 192:	0400                	addi	s0,sp,512
 194:	0510                	addi	a2,sp,640
 196:	7672                	ld	a2,312(sp)
 198:	3436                	fld	fs0,360(sp)
 19a:	3269                	addiw	tp,tp,-6
 19c:	3070                	fld	fa2,224(s0)
 19e:	6d5f 7032 5f30      	0x5f3070326d5f
 1a4:	3261                	addiw	tp,tp,-8
 1a6:	3070                	fld	fa2,224(s0)
 1a8:	635f 7032 0030      	0x307032635f
 1ae:	2a41                	addiw	s4,s4,16
 1b0:	0000                	unimp
 1b2:	7200                	ld	s0,32(a2)
 1b4:	7369                	lui	t1,0xffffa
 1b6:	01007663          	bgeu	zero,a6,1c2 <.LBB2_10+0x30>
 1ba:	0020                	addi	s0,sp,8
 1bc:	0000                	unimp
 1be:	1004                	addi	s1,sp,32
 1c0:	7205                	lui	tp,0xfffe1
 1c2:	3676                	fld	fa2,376(sp)
 1c4:	6934                	ld	a3,80(a0)
 1c6:	7032                	0x7032
 1c8:	5f30                	lw	a2,120(a4)
 1ca:	326d                	addiw	tp,tp,-5
 1cc:	3070                	fld	fa2,224(s0)
 1ce:	615f 7032 5f30      	0x5f307032615f
 1d4:	30703263          	0x30703263
 1d8:	4100                	lw	s0,0(a0)
 1da:	002a                	c.slli	zero,0xa
 1dc:	0000                	unimp
 1de:	6972                	ld	s2,280(sp)
 1e0:	00766373          	csrrsi	t1,0x7,12
 1e4:	2001                	0x2001
 1e6:	0000                	unimp
 1e8:	0400                	addi	s0,sp,512
 1ea:	0510                	addi	a2,sp,640
 1ec:	7672                	ld	a2,312(sp)
 1ee:	3436                	fld	fs0,360(sp)
 1f0:	3269                	addiw	tp,tp,-6
 1f2:	3070                	fld	fa2,224(s0)
 1f4:	6d5f 7032 5f30      	0x5f3070326d5f
 1fa:	3261                	addiw	tp,tp,-8
 1fc:	3070                	fld	fa2,224(s0)
 1fe:	635f 7032 0030      	0x307032635f
 204:	2a41                	addiw	s4,s4,16
 206:	0000                	unimp
 208:	7200                	ld	s0,32(a2)
 20a:	7369                	lui	t1,0xffffa
 20c:	01007663          	bgeu	zero,a6,218 <.LBB2_12+0x6>
 210:	0020                	addi	s0,sp,8
 212:	0000                	unimp
 214:	1004                	addi	s1,sp,32
 216:	7205                	lui	tp,0xfffe1
 218:	3676                	fld	fa2,376(sp)
 21a:	6934                	ld	a3,80(a0)
 21c:	7032                	0x7032
 21e:	5f30                	lw	a2,120(a4)
 220:	326d                	addiw	tp,tp,-5
 222:	3070                	fld	fa2,224(s0)
 224:	615f 7032 5f30      	0x5f307032615f
 22a:	30703263          	0x30703263
 22e:	4100                	lw	s0,0(a0)
 230:	002a                	c.slli	zero,0xa
 232:	0000                	unimp
 234:	6972                	ld	s2,280(sp)
 236:	00766373          	csrrsi	t1,0x7,12
 23a:	2001                	0x2001
 23c:	0000                	unimp
 23e:	0400                	addi	s0,sp,512
 240:	0510                	addi	a2,sp,640
 242:	7672                	ld	a2,312(sp)
 244:	3436                	fld	fs0,360(sp)
 246:	3269                	addiw	tp,tp,-6
 248:	3070                	fld	fa2,224(s0)
 24a:	6d5f 7032 5f30      	0x5f3070326d5f
 250:	3261                	addiw	tp,tp,-8
 252:	3070                	fld	fa2,224(s0)
 254:	635f 7032 0030      	0x307032635f
 25a:	2a41                	addiw	s4,s4,16
 25c:	0000                	unimp
 25e:	7200                	ld	s0,32(a2)
 260:	7369                	lui	t1,0xffffa
 262:	01007663          	bgeu	zero,a6,26e <.LBB2_16+0x6>
 266:	0020                	addi	s0,sp,8
 268:	0000                	unimp
 26a:	1004                	addi	s1,sp,32
 26c:	7205                	lui	tp,0xfffe1
 26e:	3676                	fld	fa2,376(sp)
 270:	6934                	ld	a3,80(a0)
 272:	7032                	0x7032
 274:	5f30                	lw	a2,120(a4)
 276:	326d                	addiw	tp,tp,-5
 278:	3070                	fld	fa2,224(s0)
 27a:	615f 7032 5f30      	0x5f307032615f
 280:	30703263          	0x30703263
 284:	4100                	lw	s0,0(a0)
 286:	002a                	c.slli	zero,0xa
 288:	0000                	unimp
 28a:	6972                	ld	s2,280(sp)
 28c:	00766373          	csrrsi	t1,0x7,12
 290:	2001                	0x2001
 292:	0000                	unimp
 294:	0400                	addi	s0,sp,512
 296:	0510                	addi	a2,sp,640
 298:	7672                	ld	a2,312(sp)
 29a:	3436                	fld	fs0,360(sp)
 29c:	3269                	addiw	tp,tp,-6
 29e:	3070                	fld	fa2,224(s0)
 2a0:	6d5f 7032 5f30      	0x5f3070326d5f
 2a6:	3261                	addiw	tp,tp,-8
 2a8:	3070                	fld	fa2,224(s0)
 2aa:	635f 7032 0030      	0x307032635f
 2b0:	2a41                	addiw	s4,s4,16
 2b2:	0000                	unimp
 2b4:	7200                	ld	s0,32(a2)
 2b6:	7369                	lui	t1,0xffffa
 2b8:	01007663          	bgeu	zero,a6,2c4 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0042b06b0b945dcfE+0x6>
 2bc:	0020                	addi	s0,sp,8
 2be:	0000                	unimp
 2c0:	1004                	addi	s1,sp,32
 2c2:	7205                	lui	tp,0xfffe1
 2c4:	3676                	fld	fa2,376(sp)
 2c6:	6934                	ld	a3,80(a0)
 2c8:	7032                	0x7032
 2ca:	5f30                	lw	a2,120(a4)
 2cc:	326d                	addiw	tp,tp,-5
 2ce:	3070                	fld	fa2,224(s0)
 2d0:	615f 7032 5f30      	0x5f307032615f
 2d6:	30703263          	0x30703263
 2da:	4100                	lw	s0,0(a0)
 2dc:	002a                	c.slli	zero,0xa
 2de:	0000                	unimp
 2e0:	6972                	ld	s2,280(sp)
 2e2:	00766373          	csrrsi	t1,0x7,12
 2e6:	2001                	0x2001
 2e8:	0000                	unimp
 2ea:	0400                	addi	s0,sp,512
 2ec:	0510                	addi	a2,sp,640
 2ee:	7672                	ld	a2,312(sp)
 2f0:	3436                	fld	fs0,360(sp)
 2f2:	3269                	addiw	tp,tp,-6
 2f4:	3070                	fld	fa2,224(s0)
 2f6:	6d5f 7032 5f30      	0x5f3070326d5f
 2fc:	3261                	addiw	tp,tp,-8
 2fe:	3070                	fld	fa2,224(s0)
 300:	635f 7032 0030      	0x307032635f
 306:	2a41                	addiw	s4,s4,16
 308:	0000                	unimp
 30a:	7200                	ld	s0,32(a2)
 30c:	7369                	lui	t1,0xffffa
 30e:	01007663          	bgeu	zero,a6,31a <.LBB0_4+0x34>
 312:	0020                	addi	s0,sp,8
 314:	0000                	unimp
 316:	1004                	addi	s1,sp,32
 318:	7205                	lui	tp,0xfffe1
 31a:	3676                	fld	fa2,376(sp)
 31c:	6934                	ld	a3,80(a0)
 31e:	7032                	0x7032
 320:	5f30                	lw	a2,120(a4)
 322:	326d                	addiw	tp,tp,-5
 324:	3070                	fld	fa2,224(s0)
 326:	615f 7032 5f30      	0x5f307032615f
 32c:	30703263          	0x30703263
 330:	4100                	lw	s0,0(a0)
 332:	002a                	c.slli	zero,0xa
 334:	0000                	unimp
 336:	6972                	ld	s2,280(sp)
 338:	00766373          	csrrsi	t1,0x7,12
 33c:	2001                	0x2001
 33e:	0000                	unimp
 340:	0400                	addi	s0,sp,512
 342:	0510                	addi	a2,sp,640
 344:	7672                	ld	a2,312(sp)
 346:	3436                	fld	fs0,360(sp)
 348:	3269                	addiw	tp,tp,-6
 34a:	3070                	fld	fa2,224(s0)
 34c:	6d5f 7032 5f30      	0x5f3070326d5f
 352:	3261                	addiw	tp,tp,-8
 354:	3070                	fld	fa2,224(s0)
 356:	635f 7032 0030      	0x307032635f
 35c:	2a41                	addiw	s4,s4,16
 35e:	0000                	unimp
 360:	7200                	ld	s0,32(a2)
 362:	7369                	lui	t1,0xffffa
 364:	01007663          	bgeu	zero,a6,370 <.LBB0_6+0x30>
 368:	0020                	addi	s0,sp,8
 36a:	0000                	unimp
 36c:	1004                	addi	s1,sp,32
 36e:	7205                	lui	tp,0xfffe1
 370:	3676                	fld	fa2,376(sp)
 372:	6934                	ld	a3,80(a0)
 374:	7032                	0x7032
 376:	5f30                	lw	a2,120(a4)
 378:	326d                	addiw	tp,tp,-5
 37a:	3070                	fld	fa2,224(s0)
 37c:	615f 7032 5f30      	0x5f307032615f
 382:	30703263          	0x30703263
 386:	4100                	lw	s0,0(a0)
 388:	002a                	c.slli	zero,0xa
 38a:	0000                	unimp
 38c:	6972                	ld	s2,280(sp)
 38e:	00766373          	csrrsi	t1,0x7,12
 392:	2001                	0x2001
 394:	0000                	unimp
 396:	0400                	addi	s0,sp,512
 398:	0510                	addi	a2,sp,640
 39a:	7672                	ld	a2,312(sp)
 39c:	3436                	fld	fs0,360(sp)
 39e:	3269                	addiw	tp,tp,-6
 3a0:	3070                	fld	fa2,224(s0)
 3a2:	6d5f 7032 5f30      	0x5f3070326d5f
 3a8:	3261                	addiw	tp,tp,-8
 3aa:	3070                	fld	fa2,224(s0)
 3ac:	635f 7032 0030      	0x307032635f
 3b2:	2a41                	addiw	s4,s4,16
 3b4:	0000                	unimp
 3b6:	7200                	ld	s0,32(a2)
 3b8:	7369                	lui	t1,0xffffa
 3ba:	01007663          	bgeu	zero,a6,3c6 <.LBB0_6+0x86>
 3be:	0020                	addi	s0,sp,8
 3c0:	0000                	unimp
 3c2:	1004                	addi	s1,sp,32
 3c4:	7205                	lui	tp,0xfffe1
 3c6:	3676                	fld	fa2,376(sp)
 3c8:	6934                	ld	a3,80(a0)
 3ca:	7032                	0x7032
 3cc:	5f30                	lw	a2,120(a4)
 3ce:	326d                	addiw	tp,tp,-5
 3d0:	3070                	fld	fa2,224(s0)
 3d2:	615f 7032 5f30      	0x5f307032615f
 3d8:	30703263          	0x30703263
 3dc:	4100                	lw	s0,0(a0)
 3de:	002a                	c.slli	zero,0xa
 3e0:	0000                	unimp
 3e2:	6972                	ld	s2,280(sp)
 3e4:	00766373          	csrrsi	t1,0x7,12
 3e8:	2001                	0x2001
 3ea:	0000                	unimp
 3ec:	0400                	addi	s0,sp,512
 3ee:	0510                	addi	a2,sp,640
 3f0:	7672                	ld	a2,312(sp)
 3f2:	3436                	fld	fs0,360(sp)
 3f4:	3269                	addiw	tp,tp,-6
 3f6:	3070                	fld	fa2,224(s0)
 3f8:	6d5f 7032 5f30      	0x5f3070326d5f
 3fe:	3261                	addiw	tp,tp,-8
 400:	3070                	fld	fa2,224(s0)
 402:	635f 7032 0030      	0x307032635f
 408:	2a41                	addiw	s4,s4,16
 40a:	0000                	unimp
 40c:	7200                	ld	s0,32(a2)
 40e:	7369                	lui	t1,0xffffa
 410:	01007663          	bgeu	zero,a6,41c <.LBB0_7+0x3c>
 414:	0020                	addi	s0,sp,8
 416:	0000                	unimp
 418:	1004                	addi	s1,sp,32
 41a:	7205                	lui	tp,0xfffe1
 41c:	3676                	fld	fa2,376(sp)
 41e:	6934                	ld	a3,80(a0)
 420:	7032                	0x7032
 422:	5f30                	lw	a2,120(a4)
 424:	326d                	addiw	tp,tp,-5
 426:	3070                	fld	fa2,224(s0)
 428:	615f 7032 5f30      	0x5f307032615f
 42e:	30703263          	0x30703263
 432:	4100                	lw	s0,0(a0)
 434:	002a                	c.slli	zero,0xa
 436:	0000                	unimp
 438:	6972                	ld	s2,280(sp)
 43a:	00766373          	csrrsi	t1,0x7,12
 43e:	2001                	0x2001
 440:	0000                	unimp
 442:	0400                	addi	s0,sp,512
 444:	0510                	addi	a2,sp,640
 446:	7672                	ld	a2,312(sp)
 448:	3436                	fld	fs0,360(sp)
 44a:	3269                	addiw	tp,tp,-6
 44c:	3070                	fld	fa2,224(s0)
 44e:	6d5f 7032 5f30      	0x5f3070326d5f
 454:	3261                	addiw	tp,tp,-8
 456:	3070                	fld	fa2,224(s0)
 458:	635f 7032 0030      	0x307032635f
 45e:	2a41                	addiw	s4,s4,16
 460:	0000                	unimp
 462:	7200                	ld	s0,32(a2)
 464:	7369                	lui	t1,0xffffa
 466:	01007663          	bgeu	zero,a6,472 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8eaf272c2eb574ceE+0x40>
 46a:	0020                	addi	s0,sp,8
 46c:	0000                	unimp
 46e:	1004                	addi	s1,sp,32
 470:	7205                	lui	tp,0xfffe1
 472:	3676                	fld	fa2,376(sp)
 474:	6934                	ld	a3,80(a0)
 476:	7032                	0x7032
 478:	5f30                	lw	a2,120(a4)
 47a:	326d                	addiw	tp,tp,-5
 47c:	3070                	fld	fa2,224(s0)
 47e:	615f 7032 5f30      	0x5f307032615f
 484:	30703263          	0x30703263
 488:	4100                	lw	s0,0(a0)
 48a:	002a                	c.slli	zero,0xa
 48c:	0000                	unimp
 48e:	6972                	ld	s2,280(sp)
 490:	00766373          	csrrsi	t1,0x7,12
 494:	2001                	0x2001
 496:	0000                	unimp
 498:	0400                	addi	s0,sp,512
 49a:	0510                	addi	a2,sp,640
 49c:	7672                	ld	a2,312(sp)
 49e:	3436                	fld	fs0,360(sp)
 4a0:	3269                	addiw	tp,tp,-6
 4a2:	3070                	fld	fa2,224(s0)
 4a4:	6d5f 7032 5f30      	0x5f3070326d5f
 4aa:	3261                	addiw	tp,tp,-8
 4ac:	3070                	fld	fa2,224(s0)
 4ae:	635f 7032 0030      	0x307032635f
 4b4:	2a41                	addiw	s4,s4,16
 4b6:	0000                	unimp
 4b8:	7200                	ld	s0,32(a2)
 4ba:	7369                	lui	t1,0xffffa
 4bc:	01007663          	bgeu	zero,a6,4c8 <_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hc899b377d5fb6a1bE+0x36>
 4c0:	0020                	addi	s0,sp,8
 4c2:	0000                	unimp
 4c4:	1004                	addi	s1,sp,32
 4c6:	7205                	lui	tp,0xfffe1
 4c8:	3676                	fld	fa2,376(sp)
 4ca:	6934                	ld	a3,80(a0)
 4cc:	7032                	0x7032
 4ce:	5f30                	lw	a2,120(a4)
 4d0:	326d                	addiw	tp,tp,-5
 4d2:	3070                	fld	fa2,224(s0)
 4d4:	615f 7032 5f30      	0x5f307032615f
 4da:	30703263          	0x30703263
 4de:	4100                	lw	s0,0(a0)
 4e0:	002a                	c.slli	zero,0xa
 4e2:	0000                	unimp
 4e4:	6972                	ld	s2,280(sp)
 4e6:	00766373          	csrrsi	t1,0x7,12
 4ea:	2001                	0x2001
 4ec:	0000                	unimp
 4ee:	0400                	addi	s0,sp,512
 4f0:	0510                	addi	a2,sp,640
 4f2:	7672                	ld	a2,312(sp)
 4f4:	3436                	fld	fs0,360(sp)
 4f6:	3269                	addiw	tp,tp,-6
 4f8:	3070                	fld	fa2,224(s0)
 4fa:	6d5f 7032 5f30      	0x5f3070326d5f
 500:	3261                	addiw	tp,tp,-8
 502:	3070                	fld	fa2,224(s0)
 504:	635f 7032 0030      	0x307032635f
 50a:	2a41                	addiw	s4,s4,16
 50c:	0000                	unimp
 50e:	7200                	ld	s0,32(a2)
 510:	7369                	lui	t1,0xffffa
 512:	01007663          	bgeu	zero,a6,51e <_ZN12tornado_user6shared15run_until_ready17he93e42bfed9e9094E+0x2c>
 516:	0020                	addi	s0,sp,8
 518:	0000                	unimp
 51a:	1004                	addi	s1,sp,32
 51c:	7205                	lui	tp,0xfffe1
 51e:	3676                	fld	fa2,376(sp)
 520:	6934                	ld	a3,80(a0)
 522:	7032                	0x7032
 524:	5f30                	lw	a2,120(a4)
 526:	326d                	addiw	tp,tp,-5
 528:	3070                	fld	fa2,224(s0)
 52a:	615f 7032 5f30      	0x5f307032615f
 530:	30703263          	0x30703263
 534:	4100                	lw	s0,0(a0)
 536:	002a                	c.slli	zero,0xa
 538:	0000                	unimp
 53a:	6972                	ld	s2,280(sp)
 53c:	00766373          	csrrsi	t1,0x7,12
 540:	2001                	0x2001
 542:	0000                	unimp
 544:	0400                	addi	s0,sp,512
 546:	0510                	addi	a2,sp,640
 548:	7672                	ld	a2,312(sp)
 54a:	3436                	fld	fs0,360(sp)
 54c:	3269                	addiw	tp,tp,-6
 54e:	3070                	fld	fa2,224(s0)
 550:	6d5f 7032 5f30      	0x5f3070326d5f
 556:	3261                	addiw	tp,tp,-8
 558:	3070                	fld	fa2,224(s0)
 55a:	635f 7032 0030      	0x307032635f
 560:	2a41                	addiw	s4,s4,16
 562:	0000                	unimp
 564:	7200                	ld	s0,32(a2)
 566:	7369                	lui	t1,0xffffa
 568:	01007663          	bgeu	zero,a6,574 <.LBB0_17+0x3c>
 56c:	0020                	addi	s0,sp,8
 56e:	0000                	unimp
 570:	1004                	addi	s1,sp,32
 572:	7205                	lui	tp,0xfffe1
 574:	3676                	fld	fa2,376(sp)
 576:	6934                	ld	a3,80(a0)
 578:	7032                	0x7032
 57a:	5f30                	lw	a2,120(a4)
 57c:	326d                	addiw	tp,tp,-5
 57e:	3070                	fld	fa2,224(s0)
 580:	615f 7032 5f30      	0x5f307032615f
 586:	30703263          	0x30703263
 58a:	4100                	lw	s0,0(a0)
 58c:	002a                	c.slli	zero,0xa
 58e:	0000                	unimp
 590:	6972                	ld	s2,280(sp)
 592:	00766373          	csrrsi	t1,0x7,12
 596:	2001                	0x2001
 598:	0000                	unimp
 59a:	0400                	addi	s0,sp,512
 59c:	0510                	addi	a2,sp,640
 59e:	7672                	ld	a2,312(sp)
 5a0:	3436                	fld	fs0,360(sp)
 5a2:	3269                	addiw	tp,tp,-6
 5a4:	3070                	fld	fa2,224(s0)
 5a6:	6d5f 7032 5f30      	0x5f3070326d5f
 5ac:	3261                	addiw	tp,tp,-8
 5ae:	3070                	fld	fa2,224(s0)
 5b0:	635f 7032 0030      	0x307032635f
 5b6:	2a41                	addiw	s4,s4,16
 5b8:	0000                	unimp
 5ba:	7200                	ld	s0,32(a2)
 5bc:	7369                	lui	t1,0xffffa
 5be:	01007663          	bgeu	zero,a6,5ca <.LBB0_17+0x92>
 5c2:	0020                	addi	s0,sp,8
 5c4:	0000                	unimp
 5c6:	1004                	addi	s1,sp,32
 5c8:	7205                	lui	tp,0xfffe1
 5ca:	3676                	fld	fa2,376(sp)
 5cc:	6934                	ld	a3,80(a0)
 5ce:	7032                	0x7032
 5d0:	5f30                	lw	a2,120(a4)
 5d2:	326d                	addiw	tp,tp,-5
 5d4:	3070                	fld	fa2,224(s0)
 5d6:	615f 7032 5f30      	0x5f307032615f
 5dc:	30703263          	0x30703263
 5e0:	4100                	lw	s0,0(a0)
 5e2:	002a                	c.slli	zero,0xa
 5e4:	0000                	unimp
 5e6:	6972                	ld	s2,280(sp)
 5e8:	00766373          	csrrsi	t1,0x7,12
 5ec:	2001                	0x2001
 5ee:	0000                	unimp
 5f0:	0400                	addi	s0,sp,512
 5f2:	0510                	addi	a2,sp,640
 5f4:	7672                	ld	a2,312(sp)
 5f6:	3436                	fld	fs0,360(sp)
 5f8:	3269                	addiw	tp,tp,-6
 5fa:	3070                	fld	fa2,224(s0)
 5fc:	6d5f 7032 5f30      	0x5f3070326d5f
 602:	3261                	addiw	tp,tp,-8
 604:	3070                	fld	fa2,224(s0)
 606:	635f 7032 0030      	0x307032635f
 60c:	2a41                	addiw	s4,s4,16
 60e:	0000                	unimp
 610:	7200                	ld	s0,32(a2)
 612:	7369                	lui	t1,0xffffa
 614:	01007663          	bgeu	zero,a6,620 <.LBB0_17+0xe8>
 618:	0020                	addi	s0,sp,8
 61a:	0000                	unimp
 61c:	1004                	addi	s1,sp,32
 61e:	7205                	lui	tp,0xfffe1
 620:	3676                	fld	fa2,376(sp)
 622:	6934                	ld	a3,80(a0)
 624:	7032                	0x7032
 626:	5f30                	lw	a2,120(a4)
 628:	326d                	addiw	tp,tp,-5
 62a:	3070                	fld	fa2,224(s0)
 62c:	615f 7032 5f30      	0x5f307032615f
 632:	30703263          	0x30703263
 636:	4100                	lw	s0,0(a0)
 638:	002a                	c.slli	zero,0xa
 63a:	0000                	unimp
 63c:	6972                	ld	s2,280(sp)
 63e:	00766373          	csrrsi	t1,0x7,12
 642:	2001                	0x2001
 644:	0000                	unimp
 646:	0400                	addi	s0,sp,512
 648:	0510                	addi	a2,sp,640
 64a:	7672                	ld	a2,312(sp)
 64c:	3436                	fld	fs0,360(sp)
 64e:	3269                	addiw	tp,tp,-6
 650:	3070                	fld	fa2,224(s0)
 652:	6d5f 7032 5f30      	0x5f3070326d5f
 658:	3261                	addiw	tp,tp,-8
 65a:	3070                	fld	fa2,224(s0)
 65c:	635f 7032 0030      	0x307032635f
 662:	2a41                	addiw	s4,s4,16
 664:	0000                	unimp
 666:	7200                	ld	s0,32(a2)
 668:	7369                	lui	t1,0xffffa
 66a:	01007663          	bgeu	zero,a6,676 <.LBB0_17+0x13e>
 66e:	0020                	addi	s0,sp,8
 670:	0000                	unimp
 672:	1004                	addi	s1,sp,32
 674:	7205                	lui	tp,0xfffe1
 676:	3676                	fld	fa2,376(sp)
 678:	6934                	ld	a3,80(a0)
 67a:	7032                	0x7032
 67c:	5f30                	lw	a2,120(a4)
 67e:	326d                	addiw	tp,tp,-5
 680:	3070                	fld	fa2,224(s0)
 682:	615f 7032 5f30      	0x5f307032615f
 688:	30703263          	0x30703263
 68c:	4100                	lw	s0,0(a0)
 68e:	002a                	c.slli	zero,0xa
 690:	0000                	unimp
 692:	6972                	ld	s2,280(sp)
 694:	00766373          	csrrsi	t1,0x7,12
 698:	2001                	0x2001
 69a:	0000                	unimp
 69c:	0400                	addi	s0,sp,512
 69e:	0510                	addi	a2,sp,640
 6a0:	7672                	ld	a2,312(sp)
 6a2:	3436                	fld	fs0,360(sp)
 6a4:	3269                	addiw	tp,tp,-6
 6a6:	3070                	fld	fa2,224(s0)
 6a8:	6d5f 7032 5f30      	0x5f3070326d5f
 6ae:	3261                	addiw	tp,tp,-8
 6b0:	3070                	fld	fa2,224(s0)
 6b2:	635f 7032 0030      	0x307032635f
 6b8:	2a41                	addiw	s4,s4,16
 6ba:	0000                	unimp
 6bc:	7200                	ld	s0,32(a2)
 6be:	7369                	lui	t1,0xffffa
 6c0:	01007663          	bgeu	zero,a6,6cc <_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17haef57489042b0ba3E+0x44>
 6c4:	0020                	addi	s0,sp,8
 6c6:	0000                	unimp
 6c8:	1004                	addi	s1,sp,32
 6ca:	7205                	lui	tp,0xfffe1
 6cc:	3676                	fld	fa2,376(sp)
 6ce:	6934                	ld	a3,80(a0)
 6d0:	7032                	0x7032
 6d2:	5f30                	lw	a2,120(a4)
 6d4:	326d                	addiw	tp,tp,-5
 6d6:	3070                	fld	fa2,224(s0)
 6d8:	615f 7032 5f30      	0x5f307032615f
 6de:	30703263          	0x30703263
 6e2:	4100                	lw	s0,0(a0)
 6e4:	002a                	c.slli	zero,0xa
 6e6:	0000                	unimp
 6e8:	6972                	ld	s2,280(sp)
 6ea:	00766373          	csrrsi	t1,0x7,12
 6ee:	2001                	0x2001
 6f0:	0000                	unimp
 6f2:	0400                	addi	s0,sp,512
 6f4:	0510                	addi	a2,sp,640
 6f6:	7672                	ld	a2,312(sp)
 6f8:	3436                	fld	fs0,360(sp)
 6fa:	3269                	addiw	tp,tp,-6
 6fc:	3070                	fld	fa2,224(s0)
 6fe:	6d5f 7032 5f30      	0x5f3070326d5f
 704:	3261                	addiw	tp,tp,-8
 706:	3070                	fld	fa2,224(s0)
 708:	635f 7032 0030      	0x307032635f

Disassembly of section .comment:

0000000000000000 <.comment>:
   0:	694c                	ld	a1,144(a0)
   2:	6b6e                	ld	s6,216(sp)
   4:	7265                	lui	tp,0xffff9
   6:	203a                	fld	ft0,392(sp)
   8:	4c4c                	lw	a1,28(s0)
   a:	2044                	fld	fs1,128(s0)
   c:	3131                	addiw	sp,sp,-20
   e:	302e                	fld	ft0,232(sp)
  10:	312e                	fld	ft2,232(sp)
	...

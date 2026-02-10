# Liquidation Log Analysis (slot 399316315 / 399316316 / 399316345)

## 1. 原始日志（完整）

```text
liq detail: user_account=AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v authority=3b2UdoXUEjYTPxFMMiUtZcNJzKDmFHt1B2wDMy34rnTb sub_account_id=0 margin_mode=HighLeverage pool_id=0 max_margin_ratio=0 last_active_slot=399316345 open_orders=0 has_open_order=false user_age_slots=0 margin_buffer_ratio=100
liq detail: ffi total_collateral=42650259 margin_requirement=42918888 total_collateral_buffer=-479489 margin_requirement_plus_buffer=90322260 free_collateral=-268629 cross_status=Liquidatable isolated_count=0
liq detail: isolated summary user_perp_iso_count=0 user_perp_iso_markets=[] ffi_iso_count=0 with_perp_iso_liability=false with_spot_iso_liability=false
liq detail: spot pos idx=0 balance=Borrow scaled_balance=8411071425 open_orders=0 open_bids=0 open_asks=0
liq detail: spot pos idx=1 balance=Deposit scaled_balance=1103590647 open_orders=0 open_bids=0 open_asks=0
liq detail: perp pos idx=1 base_asset_amount=68900000 quote_entry_amount=-4816110000 quote_asset_amount=-4777182968 open_orders=0 open_bids=0 open_asks=0 lp_shares=0 isolated_scaled_balance=0 max_margin_ratio=109 position_flag=0
liq detail: spot market idx=1 oracle=3m6i4RFWEDw2Ft4tFHPJtYgmpPe21k56M3FHeWYrgGBz oracle_source=PythLazer decimals=9 asset_tier=Collateral
liq detail: spot oracle cache idx=1 price=83866178 conf=18705 delay=0 has_data_points=true seq_id=Some(1770723859200000) age_slots=1
liq detail: spot oracle state idx=1 price=83866178 conf=18705 delay=0 has_data_points=true seq_id=Some(1770723859200000)
liq detail: spot market idx=0 oracle=9VCioxmni2gDLv11qufWzT3RDERhQE4iY5Gf7NTfYyAV oracle_source=PythLazerStableCoin decimals=6 asset_tier=Collateral
liq detail: spot oracle cache idx=0 price=999873 conf=22 delay=0 has_data_points=true seq_id=Some(1770723846200000) age_slots=33
liq detail: spot oracle state idx=0 price=999873 conf=22 delay=0 has_data_points=true seq_id=Some(1770723846200000)
liq detail: perp market idx=1 oracle=35MbvS1Juz2wf7GsyHrkCw8yfKciRLxVpEhfZDZFrB4R oracle_source=PythLazer quote_spot_market_index=0 contract_tier=A
liq detail: perp oracle cache idx=1 price=68639012336 conf=1922941 delay=0 has_data_points=true seq_id=Some(1770723859200000) age_slots=1
liq detail: perp oracle state idx=1 price=68639012336 conf=1922941 delay=0 has_data_points=true seq_id=Some(1770723859200000)
{"event":"liq_perp_submit","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","slot":399316315}
{"event":"liq_perp_send_failed","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","observed_slot":399316315,"error":"RPC response error -32002: Transaction simulation failed: Error processing Instruction 2: custom program error: 0x1774; 10 log messages:
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH invoke [1]
  Program log: Instruction: LiquidatePerp
  Program log: margin calculation: MarginCalculation { context: MarginContext { margin_type: Maintenance, mode: Liquidation { market_to_track_margin_requirement: Some(MarketIdentifier { market_type: Perp, market_index: 1 }) }, strict: false, ignore_invalid_deposit_oracles: false, margin_buffer: 100, fuel_bonus_numerator: 0, fuel_bonus: 0, fuel_perp_delta: None, fuel_spot_deltas: [(0, 0), (0, 0)], margin_ratio_override: None }, total_collateral: 62251506, total_collateral_buffer: -283476, margin_requirement: 56598278, margin_requirement_plus_buffer: 124727999, isolated_margin_calculations: {}, num_spot_liabilities: 1, num_perp_liabilities: 1, all_deposit_oracles_valid: true, all_liability_oracles_valid: true, with_perp_isolated_liability: false, with_spot_isolated_liability: false, total_spot_asset_value: 0, total_spot_liability_value: 0, total_perp_liability_value: 0, total_perp_pnl: 0, tracked_market_margin_requirement: 44888330, fuel_deposits: 0, fuel_borrows: 0, fuel_positions: 0 }
  Program log: AnchorError occurred. Error Code: SufficientCollateral. Error Number: 6004. Error Message: Sufficient collateral.
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH consumed 61566 of 255700 compute units
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH failed: custom program error: 0x1774
"}
{"event":"liq_perp_submit","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","slot":399316315}
{"event":"liq_perp_jito_send_failed","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","observed_slot":399316315,"error":"jito send failed: jito status=429 Too Many Requests body={"jsonrpc":"2.0","error":{"code":-32097,"message":"Network congested. Endpoint is globally rate limited.","data":null},"id":null}"}
{"event":"liq_perp_send_failed","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","observed_slot":399316315,"error":"RPC response error -32002: Transaction simulation failed: Error processing Instruction 2: custom program error: 0x1774; 10 log messages:
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH invoke [1]
  Program log: Instruction: LiquidatePerp
  Program log: margin calculation: MarginCalculation { context: MarginContext { margin_type: Maintenance, mode: Liquidation { market_to_track_margin_requirement: Some(MarketIdentifier { market_type: Perp, market_index: 1 }) }, strict: false, ignore_invalid_deposit_oracles: false, margin_buffer: 100, fuel_bonus_numerator: 0, fuel_bonus: 0, fuel_perp_delta: None, fuel_spot_deltas: [(0, 0), (0, 0)], margin_ratio_override: None }, total_collateral: 62178336, total_collateral_buffer: -284202, margin_requirement: 56597800, margin_requirement_plus_buffer: 124726796, isolated_margin_calculations: {}, num_spot_liabilities: 1, num_perp_liabilities: 1, all_deposit_oracles_valid: true, all_liability_oracles_valid: true, with_perp_isolated_liability: false, with_spot_isolated_liability: false, total_spot_asset_value: 0, total_spot_liability_value: 0, total_perp_liability_value: 0, total_perp_pnl: 0, tracked_market_margin_requirement: 44887852, fuel_deposits: 0, fuel_borrows: 0, fuel_positions: 0 }
  Program log: AnchorError occurred. Error Code: SufficientCollateral. Error Number: 6004. Error Message: Sufficient collateral.
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH consumed 61566 of 255700 compute units
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH failed: custom program error: 0x1774
"}
{"event":"liq_perp_submit","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","slot":399316315}
{"event":"liq_perp_send_failed","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","observed_slot":399316315,"error":"RPC response error -32002: Transaction simulation failed: Error processing Instruction 2: custom program error: 0x1774; 10 log messages:
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH invoke [1]
  Program log: Instruction: LiquidatePerp
  Program log: margin calculation: MarginCalculation { context: MarginContext { margin_type: Maintenance, mode: Liquidation { market_to_track_margin_requirement: Some(MarketIdentifier { market_type: Perp, market_index: 1 }) }, strict: false, ignore_invalid_deposit_oracles: false, margin_buffer: 100, fuel_bonus_numerator: 0, fuel_bonus: 0, fuel_perp_delta: None, fuel_spot_deltas: [(0, 0), (0, 0)], margin_ratio_override: None }, total_collateral: 62178336, total_collateral_buffer: -284202, margin_requirement: 56597800, margin_requirement_plus_buffer: 124726796, isolated_margin_calculations: {}, num_spot_liabilities: 1, num_perp_liabilities: 1, all_deposit_oracles_valid: true, all_liability_oracles_valid: true, with_perp_isolated_liability: false, with_spot_isolated_liability: false, total_spot_asset_value: 0, total_spot_liability_value: 0, total_perp_liability_value: 0, total_perp_pnl: 0, tracked_market_margin_requirement: 44887852, fuel_deposits: 0, fuel_borrows: 0, fuel_positions: 0 }
  Program log: AnchorError occurred. Error Code: SufficientCollateral. Error Number: 6004. Error Message: Sufficient collateral.
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH consumed 61566 of 255700 compute units
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH failed: custom program error: 0x1774
"}
liq detail: user_account=AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v authority=3b2UdoXUEjYTPxFMMiUtZcNJzKDmFHt1B2wDMy34rnTb sub_account_id=0 margin_mode=HighLeverage pool_id=0 max_margin_ratio=0 last_active_slot=399316345 open_orders=0 has_open_order=false user_age_slots=0 margin_buffer_ratio=100
liq detail: ffi total_collateral=39331150 margin_requirement=40653780 total_collateral_buffer=-512673 margin_requirement_plus_buffer=84625169 free_collateral=-1322630 cross_status=Liquidatable isolated_count=0
liq detail: isolated summary user_perp_iso_count=0 user_perp_iso_markets=[] ffi_iso_count=0 with_perp_iso_liability=false with_spot_iso_liability=false
liq detail: spot pos idx=0 balance=Borrow scaled_balance=8411071425 open_orders=0 open_bids=0 open_asks=0
liq detail: spot pos idx=1 balance=Deposit scaled_balance=1103590647 open_orders=0 open_bids=0 open_asks=0
liq detail: perp pos idx=1 base_asset_amount=63900000 quote_entry_amount=-4466610000 quote_asset_amount=-4437260021 open_orders=0 open_bids=0 open_asks=0 lp_shares=0 isolated_scaled_balance=0 max_margin_ratio=109 position_flag=0
liq detail: spot market idx=0 oracle=9VCioxmni2gDLv11qufWzT3RDERhQE4iY5Gf7NTfYyAV oracle_source=PythLazerStableCoin decimals=6 asset_tier=Collateral
liq detail: spot oracle cache idx=0 price=999873 conf=22 delay=0 has_data_points=true seq_id=Some(1770723846200000) age_slots=34
liq detail: spot oracle state idx=0 price=999873 conf=22 delay=0 has_data_points=true seq_id=Some(1770723846200000)
liq detail: spot market idx=1 oracle=3m6i4RFWEDw2Ft4tFHPJtYgmpPe21k56M3FHeWYrgGBz oracle_source=PythLazer decimals=9 asset_tier=Collateral
liq detail: spot oracle cache idx=1 price=83865580 conf=14065 delay=0 has_data_points=true seq_id=Some(1770723859600000) age_slots=1
liq detail: spot oracle state idx=1 price=83865580 conf=14065 delay=0 has_data_points=true seq_id=Some(1770723859600000)
liq detail: perp market idx=1 oracle=35MbvS1Juz2wf7GsyHrkCw8yfKciRLxVpEhfZDZFrB4R oracle_source=PythLazer quote_spot_market_index=0 contract_tier=A
liq detail: perp oracle cache idx=1 price=68638280420 conf=1105336 delay=0 has_data_points=true seq_id=Some(1770723859600000) age_slots=1
liq detail: perp oracle state idx=1 price=68638280420 conf=1105336 delay=0 has_data_points=true seq_id=Some(1770723859600000)
{"event":"liq_perp_submit","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","slot":399316316}
{"event":"liq_perp_jito_send_failed","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","observed_slot":399316316,"error":"jito send failed: jito status=429 Too Many Requests body={"jsonrpc":"2.0","error":{"code":-32097,"message":"Network congested. Endpoint is globally rate limited.","data":null},"id":null}"}
{"event":"liq_perp_send_failed","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","observed_slot":399316316,"error":"RPC response error -32002: Transaction simulation failed: Error processing Instruction 2: custom program error: 0x1774; 10 log messages:
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH invoke [1]
  Program log: Instruction: LiquidatePerp
  Program log: margin calculation: MarginCalculation { context: MarginContext { margin_type: Maintenance, mode: Liquidation { market_to_track_margin_requirement: Some(MarketIdentifier { market_type: Perp, market_index: 1 }) }, strict: false, ignore_invalid_deposit_oracles: false, margin_buffer: 100, fuel_bonus_numerator: 0, fuel_bonus: 0, fuel_perp_delta: None, fuel_spot_deltas: [(0, 0), (0, 0)], margin_ratio_override: None }, total_collateral: 62178336, total_collateral_buffer: -284202, margin_requirement: 56597800, margin_requirement_plus_buffer: 124726796, isolated_margin_calculations: {}, num_spot_liabilities: 1, num_perp_liabilities: 1, all_deposit_oracles_valid: true, all_liability_oracles_valid: true, with_perp_isolated_liability: false, with_spot_isolated_liability: false, total_spot_asset_value: 0, total_spot_liability_value: 0, total_perp_liability_value: 0, total_perp_pnl: 0, tracked_market_margin_requirement: 44887852, fuel_deposits: 0, fuel_borrows: 0, fuel_positions: 0 }
  Program log: AnchorError occurred. Error Code: SufficientCollateral. Error Number: 6004. Error Message: Sufficient collateral.
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH consumed 61566 of 255700 compute units
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH failed: custom program error: 0x1774
"}
{"event":"liq_perp_submit","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","slot":399316316}
{"event":"liq_perp_send_failed","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","observed_slot":399316316,"error":"RPC response error -32002: Transaction simulation failed: Error processing Instruction 2: custom program error: 0x1774; 10 log messages:
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH invoke [1]
  Program log: Instruction: LiquidatePerp
  Program log: margin calculation: MarginCalculation { context: MarginContext { margin_type: Maintenance, mode: Liquidation { market_to_track_margin_requirement: Some(MarketIdentifier { market_type: Perp, market_index: 1 }) }, strict: false, ignore_invalid_deposit_oracles: false, margin_buffer: 100, fuel_bonus_numerator: 0, fuel_bonus: 0, fuel_perp_delta: None, fuel_spot_deltas: [(0, 0), (0, 0)], margin_ratio_override: None }, total_collateral: 62284055, total_collateral_buffer: -283144, margin_requirement: 56598498, margin_requirement_plus_buffer: 124728552, isolated_margin_calculations: {}, num_spot_liabilities: 1, num_perp_liabilities: 1, all_deposit_oracles_valid: true, all_liability_oracles_valid: true, with_perp_isolated_liability: false, with_spot_isolated_liability: false, total_spot_asset_value: 0, total_spot_liability_value: 0, total_perp_liability_value: 0, total_perp_pnl: 0, tracked_market_margin_requirement: 44888550, fuel_deposits: 0, fuel_borrows: 0, fuel_positions: 0 }
  Program log: AnchorError occurred. Error Code: SufficientCollateral. Error Number: 6004. Error Message: Sufficient collateral.
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH consumed 61566 of 255700 compute units
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH failed: custom program error: 0x1774
"}
{"event":"liq_perp_submit","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","slot":399316316}
{"event":"liq_perp_jito_send_failed","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","observed_slot":399316316,"error":"jito send failed: jito status=429 Too Many Requests body={"jsonrpc":"2.0","error":{"code":-32097,"message":"Network congested. Endpoint is globally rate limited.","data":null},"id":null}"}
{"event":"liq_perp_send_failed","market_index":1,"liquidatee":"AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v","observed_slot":399316316,"error":"RPC response error -32002: Transaction simulation failed: Error processing Instruction 2: custom program error: 0x1774; 10 log messages:
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program ComputeBudget111111111111111111111111111111 invoke [1]
  Program ComputeBudget111111111111111111111111111111 success
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH invoke [1]
  Program log: Instruction: LiquidatePerp
  Program log: margin calculation: MarginCalculation { context: MarginContext { margin_type: Maintenance, mode: Liquidation { market_to_track_margin_requirement: Some(MarketIdentifier { market_type: Perp, market_index: 1 }) }, strict: false, ignore_invalid_deposit_oracles: false, margin_buffer: 100, fuel_bonus_numerator: 0, fuel_bonus: 0, fuel_perp_delta: None, fuel_spot_deltas: [(0, 0), (0, 0)], margin_ratio_override: None }, total_collateral: 62284055, total_collateral_buffer: -283144, margin_requirement: 56598498, margin_requirement_plus_buffer: 124728552, isolated_margin_calculations: {}, num_spot_liabilities: 1, num_perp_liabilities: 1, all_deposit_oracles_valid: true, all_liability_oracles_valid: true, with_perp_isolated_liability: false, with_spot_isolated_liability: false, total_spot_asset_value: 0, total_spot_liability_value: 0, total_perp_liability_value: 0, total_perp_pnl: 0, tracked_market_margin_requirement: 44888550, fuel_deposits: 0, fuel_borrows: 0, fuel_positions: 0 }
  Program log: AnchorError occurred. Error Code: SufficientCollateral. Error Number: 6004. Error Message: Sufficient collateral.
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH consumed 61566 of 255700 compute units
  Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH failed: custom program error: 0x1774
"}
liq detail: user_account=AnFfHkpmoHuLnKMaygw8URgvhHcDaBKHw9dUSn1KxE1v authority=3b2UdoXUEjYTPxFMMiUtZcNJzKDmFHt1B2wDMy34rnTb sub_account_id=0 margin_mode=HighLeverage pool_id=0 max_margin_ratio=0 last_active_slot=399316345 open_orders=0 has_open_order=false user_age_slots=0 margin_buffer_ratio=100
liq detail: ffi total_collateral=-2113057 margin_requirement=11709948 total_collateral_buffer=-927115 margin_requirement_plus_buffer=11827047 free_collateral=-13823005 cross_status=Liquidatable isolated_count=0
liq detail: isolated summary user_perp_iso_count=0 user_perp_iso_markets=[] ffi_iso_count=0 with_perp_iso_liability=false with_spot_iso_liability=false
liq detail: spot pos idx=0 balance=Borrow scaled_balance=8411071425 open_orders=0 open_bids=0 open_asks=0
liq detail: spot pos idx=1 balance=Deposit scaled_balance=1103590647 open_orders=0 open_bids=0 open_asks=0
liq detail: perp pos idx=1 base_asset_amount=0 quote_entry_amount=0 quote_asset_amount=-92723297 open_orders=0 open_bids=0 open_asks=0 lp_shares=0 isolated_scaled_balance=0 max_margin_ratio=109 position_flag=0
liq detail: spot market idx=1 oracle=3m6i4RFWEDw2Ft4tFHPJtYgmpPe21k56M3FHeWYrgGBz oracle_source=PythLazer decimals=9 asset_tier=Collateral
liq detail: spot oracle cache idx=1 price=83865508 conf=19377 delay=0 has_data_points=true seq_id=Some(1770723860000000) age_slots=1
liq detail: spot oracle state idx=1 price=83865508 conf=19377 delay=0 has_data_points=true seq_id=Some(1770723860000000)
liq detail: spot market idx=0 oracle=9VCioxmni2gDLv11qufWzT3RDERhQE4iY5Gf7NTfYyAV oracle_source=PythLazerStableCoin decimals=6 asset_tier=Collateral
liq detail: spot oracle cache idx=0 price=999873 conf=22 delay=0 has_data_points=true seq_id=Some(1770723846200000) age_slots=35
liq detail: spot oracle state idx=0 price=999873 conf=22 delay=0 has_data_points=true seq_id=Some(1770723846200000)
liq detail: perp market idx=1 oracle=35MbvS1Juz2wf7GsyHrkCw8yfKciRLxVpEhfZDZFrB4R oracle_source=PythLazer quote_spot_market_index=0 contract_tier=A
liq detail: perp oracle cache idx=1 price=68639348115 conf=1922940 delay=0 has_data_points=true seq_id=Some(1770723860000000) age_slots=1
liq detail: perp oracle state idx=1 price=68639348115 conf=1922940 delay=0 has_data_points=true seq_id=Some(1770723860000000)
```

## 2. 详细分析报告

### 2.1 先给结论

这段日志里看见的“slot 不一致”，核心不是单一字段读错，而是以下三件事叠加：

1. 不同字段是不同语义：
   - `last_active_slot` 是用户账户字段（账户最近活跃槽位）。
   - `liq_perp_submit.slot` / `observed_slot` 是 liquidator 入队时记录的观测槽位。
2. 同一个机会会有多次重试（从日志可见至少 3 次 submit）。
3. 多个异步任务并发写日志，文件行序不一定等于严格因果序。

因此，出现“detail 里看到更大的 last_active_slot，但 submit 仍是较小 slot”的现象是可能的。

---

### 2.2 为什么你会感觉“明明 399316345 发现，却按 399316315 下单”

从日志本身看，`last_active_slot=399316345` 并不是“本次扫描机会的 slot”；它是用户状态字段。该字段可以大于当前 liquidator 的某次观测 slot，不代表这次机会是在 399316345 才被发现。

并且，这段日志里有两批 submit：
- 一批是 `slot=399316315`
- 一批是 `slot=399316316`

说明 liquidator 在相邻 slot 连续尝试了机会，而不是只发生一次。

---

### 2.3 按时间顺序重建过程（从你贴的内容）

1. **第一次 detail 快照**
   - `base_asset_amount=68900000`（有 perp 仓位）
   - `cross_status=Liquidatable`
   - 然后进入 `slot=399316315` 的多次 `liq_perp_submit`

2. **slot=399316315 的发送结果**
   - 多次 `liq_perp_send_failed`
   - 失败主因固定：
     - 程序返回 `0x1774`
     - Anchor 错误 `6004 SufficientCollateral`
   - 含义：**链上执行时判定“并不满足 perp 清算条件”**。

3. **第二次 detail 快照**
   - `base_asset_amount` 从 68900000 降到 63900000（仓位变化）
   - 依然 `cross_status=Liquidatable`
   - 随后出现 `slot=399316316` 的新一批 submit（又是多次）

4. **slot=399316316 的发送结果**
   - 继续 `6004 SufficientCollateral`
   - 同时出现 `jito 429`（拥堵/限流）

5. **第三次 detail 快照**
   - `base_asset_amount=0`（perp 仓位已无）
   - 但账户仍 `cross_status=Liquidatable`
   - 说明账户层面仍有风险（例如 spot 借贷导致），但这一路 `liq_perp` 已不再匹配可执行目标。

---

### 2.4 为什么会“发现可清算却链上说 SufficientCollateral”

这在高竞争场景很常见，根因通常是“状态在你发交易前已变化”：

- 你本地判定基于某个瞬时快照。
- 交易送到 RPC 模拟时，账户、oracle、仓位、他人清算进度已变化。
- 结果变成：本地看起来可清算，链上执行瞬间不可清算，返回 `6004`。

日志中 `margin calculation` 每次的 `total_collateral/margin_requirement` 数值都在变，也支持这个结论。

---

### 2.5 关于“slot 读错/BUG”的判断

仅凭这段日志，**不能直接证明**“slot 读取有 bug”。

要严格证明，需要同一机会全链路关联字段：

- `op_id`：扫描时生成，贯穿 detail -> enqueue -> submit -> send_failed
- `scan_slot`：扫描判定当时的 slot
- `enqueue_slot`
- `submit_slot_now`：真正发送前再读一次 slot
- `scan_ts_ms / enqueue_ts_ms / submit_ts_ms`

如果未来抓到同一个 `op_id` 出现 `submit_slot_now < scan_slot`，那才是硬证据。

---

### 2.6 这段日志反映出的真实问题（比“slot不一致”更关键）

1. `liq_perp` 在短时间内重复提交多次，但全部被 `6004` 拒绝，说明策略对“瞬时可清算窗口”命中不稳。
2. Jito 通道有 429，意味着发送通道在拥堵期不可用或配额不足。
3. 后续账户仍 Liquidatable 但 perp=0，说明应该切换或补充非-perp 清算路径，否则会“看见机会但打不到”。

---

### 2.7 建议的工程化修复

1. 统一日志语义：把 `last_active_slot` 重命名显示为 `user_last_active_slot`，避免误读为“本次扫描slot”。
2. 加 `op_id` + 三段时间戳 + 三段 slot（scan/enqueue/submit_now）。
3. 在 tx_worker 发送前增加一次快速预检查（最新账户快照）以减少无效发送。
4. 对 `6004` 做短期抑制（同用户同市场在 N ms 内降频），避免重复打空。
5. 对 Jito 429 做退避 + 降级通道策略（快速切 RPC 或分流 endpoint）。

## 3. 最终结论

- 这段日志中“399316345 vs 399316315/316”的不一致，主要来自字段语义不同与异步流水线，不足以单独定性为 slot 读取 bug。
- 真正可确认的问题是：**机会窗口极短 + 重试路径命中率低 + 发送通道拥堵**，导致多次提交但链上判定不可清算。
- 若要彻底判责“是否 slot 读取错误”，必须补上 `op_id + scan/enqueue/submit_now` 的同机会关联日志。

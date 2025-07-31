# predictor.py

import pandas as pd
import xgboost as xgb
import numpy as np

# 데이터 전처리 함수
def prepare_data(past_draws):
    df = pd.DataFrame(past_draws, columns=[f'N{i+1}' for i in range(6)])

    # 각 번호를 원-핫 인코딩하여 타겟 데이터 생성
    all_nums = range(1, 46)
    targets = []
    for row in df.itertuples(index=False):
        target = [1 if i in row else 0 for i in all_nums]
        targets.append(target)
    Y = np.array(targets)

    # 피처는 이전 회차 번호들
    # 예시: 바로 전 회차의 번호들만 사용
    X = df.shift(1).dropna().astype(int).values.tolist()
    Y = Y[1:]  # 타겟도 맞춰서 하나 줄임

    return np.array(X), np.array(Y)

# 예측 함수
def predict(past_draws):
    X, Y = prepare_data(past_draws)

    if len(X) < 5:
        raise ValueError("데이터가 충분하지 않습니다 (최소 5회차 필요)")

    model = xgb.XGBClassifier(
        objective='binary:logistic',
        eval_metric='logloss'
    )
    model.fit(X, Y)

    # 가장 최근 회차를 기준으로 다음 번호 예측
    latest = np.array(past_draws[-1]).reshape(1, -1)
    probs = model.predict_proba(latest)

    # XGBoost에서는 각 열(1~45)에 대해 확률 반환
    if isinstance(probs, list):  # XGBoost 1.7+ 버전 호환성 처리
        probs = np.array([p[1] for p in probs]).reshape(1, -1)

    probabilities = probs[0]
    top_6 = sorted(np.argsort(probabilities)[-6:] + 1)  # 0부터 시작하므로 +1
    return top_6

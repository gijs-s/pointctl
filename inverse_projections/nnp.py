import numpy as np
from sklearn import decomposition, preprocessing
from tensorflow.keras import backend as K
from tensorflow.keras.callbacks import EarlyStopping
from tensorflow.keras.initializers import Constant
from tensorflow.keras.layers import Dense, Dropout
from tensorflow.keras.models import Sequential
from tensorflow.keras import regularizers, optimizers

class NNP():
    def __init__(self, seed_data='projection', init=decomposition.PCA(n_components=2), size='medium', style='bottleneck', loss='mean_absolute_error', epochs=1000, scale_data=True, opt='adam', l1=0.0, l2=0.0, dropout=True, verbose=0):
        assert (seed_data == 'projection') or (seed_data == 'precomputed')

        self.nnsettings = dict()

        self.nnsettings['small'] = dict()
        self.nnsettings['medium'] = dict()
        self.nnsettings['large'] = dict()
        self.nnsettings['std'] = dict()

        self.nnsettings['std']['wide'] = [256,512,256]

        self.nnsettings['small']['straight'] = [120,120,120]
        self.nnsettings['small']['wide'] = [90,180,90]
        self.nnsettings['small']['bottleneck'] = [150,60,150]

        self.nnsettings['medium']['straight'] = [240,240,240]
        self.nnsettings['medium']['wide'] = [180,360,180]
        self.nnsettings['medium']['bottleneck'] = [300,120,300]

        self.nnsettings['large']['straight'] = [480,480,480]
        self.nnsettings['large']['wide'] = [360,720,360]
        self.nnsettings['large']['bottleneck'] = [600,240,600]
        self.layers = self.nnsettings[size][style]

        self.stop = EarlyStopping(verbose=verbose, min_delta=0.00001, mode='min', patience=10, restore_best_weights=True)
        self.callbacks = [self.stop]

        self.init = init
        self.dropout = dropout
        self.scale_data = scale_data
        self.opt = opt
        self.epochs = epochs
        self.loss = loss
        self.l1 = l1
        self.l2 = l2
        self.verbose = verbose

        if seed_data == 'precomputed':
            self.X_2d = init
        else:
            self.X_2d = None

        self.scaler = preprocessing.MinMaxScaler()

        self.is_fitted = False
        K.clear_session()

    def fit(self, X):
        self.model = Sequential()
        self.model.add(Dense(self.layers[0], activation='relu',
                    kernel_initializer='he_uniform',
                    bias_initializer=Constant(0.0001),
                    input_shape=(X.shape[1],)))
        self.model.add(Dense(self.layers[1], activation='relu',
                    kernel_regularizer=regularizers.l1_l2(l1=self.l1, l2=self.l2),
                    kernel_initializer='he_uniform',
                    bias_initializer=Constant(0.0001)))
        self.model.add(Dense(self.layers[2], activation='relu',
                    kernel_initializer='he_uniform',
                    bias_initializer=Constant(0.0001)))
        if self.dropout:
            self.model.add(Dropout(0.5))

        self.model.add(Dense(2, activation='sigmoid',
                    kernel_initializer='he_uniform',
                    bias_initializer=Constant(0.0001)))
        self.model.compile(loss=self.loss, optimizer=self.opt)

        if self.X_2d is None:
            X_2d = self.init.fit_transform(X)
        else:
            X_2d = self.X_2d

        if self.scale_data:
            X_2d = self.scaler.fit_transform(X_2d)

        self.model.fit(X, X_2d, batch_size=32, epochs=self.epochs, verbose=0, validation_split=0.05, callbacks=self.callbacks)
        self.is_fitted = True

    def _is_fit(self):
        if self.is_fitted:
            return True
        else:
            raise Exception('Model not trained. Call fit() before calling transform()')

    def transform(self, X):
        if self._is_fit():
            X_2d = self.model.predict(X)

            if self.scale_data:
                X_2d = self.scaler.inverse_transform(X_2d)

            return X_2d


class NNInv():
    def __init__(self, loss='mean_absolute_error', size='medium', style='funnel', epochs=300, scale_data=True, verbose=0):
        self.nnsettings = dict()

        self.nnsettings['small'] = dict()
        self.nnsettings['medium'] = dict()
        self.nnsettings['large'] = dict()
        self.nnsettings['xlarge'] = dict()

        self.nnsettings['large']['bottleneck'] = [2048, 512, 512, 2048]
        self.nnsettings['large']['funnel'] = [256, 512, 2048, 4096]
        self.nnsettings['large']['straight'] = [1024, 1024, 1024, 1024]
        self.nnsettings['large']['wide'] = [512, 2048, 2048, 512]
        self.nnsettings['medium']['bottleneck'] = [1024, 256, 256, 1024]
        self.nnsettings['medium']['funnel'] = [128, 256, 1024, 2048]
        self.nnsettings['medium']['straight'] = [512, 512, 512, 512]
        self.nnsettings['medium']['wide'] = [256, 1024, 1024, 256]
        self.nnsettings['small']['bottleneck'] = [512, 128, 128, 512]
        self.nnsettings['small']['funnel'] = [64, 128, 512, 1024]
        self.nnsettings['small']['straight'] = [256, 256, 256, 256]
        self.nnsettings['small']['wide'] = [128, 512, 512, 128]
        self.nnsettings['xlarge']['bottleneck'] = [4096, 1024, 1024, 4096]
        self.nnsettings['xlarge']['funnel'] = [512, 1024, 4096, 8192]
        self.nnsettings['xlarge']['straight'] = [2048, 2048, 2048, 2048]
        self.nnsettings['xlarge']['wide'] = [1024, 4096, 4096, 1024]

        self.layers = self.nnsettings[size][style]

        self.loss = loss
        self.epochs = epochs
        self.scale_data = scale_data
        self.verbose = verbose
        self.scaler = preprocessing.MinMaxScaler()

        self.stop = EarlyStopping(verbose=verbose, min_delta=0.00001, mode='min', patience=10, restore_best_weights=True)
        self.callbacks = [self.stop]

        self.is_fitted = False
        K.clear_session()

    def _is_fit(self):
        if self.is_fitted:
            return True
        else:
            raise Exception('Model not trained. Call fit() before calling inverse_transform()')

    def fit(self, X, X_2d):
        self.m = Sequential()
        self.m.add(Dense(self.layers[0], activation='relu', kernel_initializer='he_uniform', bias_initializer=Constant(0.01), input_shape=(X_2d.shape[1],)))
        self.m.add(Dense(self.layers[1], activation='relu', kernel_initializer='he_uniform', bias_initializer=Constant(0.01)))
        self.m.add(Dense(self.layers[2], activation='relu', kernel_initializer='he_uniform', bias_initializer=Constant(0.01)))
        self.m.add(Dense(self.layers[3], activation='relu', kernel_initializer='he_uniform', bias_initializer=Constant(0.01)))
        self.m.add(Dense(X.shape[1], activation='sigmoid', kernel_initializer='he_uniform'))
        self.m.compile(loss=self.loss, optimizer='adam')

        if self.scale_data:
            X_2d_ = self.scaler.fit_transform(X_2d)
        else:
            X_2d_ = X_2d

        self.m.fit(X_2d_, X, batch_size=32, epochs=self.epochs, verbose=self.verbose, validation_split=0.05, callbacks=self.callbacks)
        self.is_fitted = True

    def inverse_transform(self, X_2d):
        if self._is_fit():
            if self.scale_data:
                X_2d_ = self.scaler.transform(X_2d)
            else:
                X_2d_ = X_2d

            return self.m.predict(X_2d_)